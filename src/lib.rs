use std::io::{self, prelude::*};
use std::net;

use crate::binary::{Encoder, ReadEx};
use crate::types::{Block, ResultWriter};

use chrono_tz::Tz;
use errors::Result;
use log::debug;
use protocols::{
    ExceptionResponse, HelloRequest, HelloResponse, QueryProtocol, SERVER_END_OF_STREAM,
};

mod binary;
pub mod error_codes;
pub mod errors;
pub mod protocols;
pub mod types;

#[macro_use]
extern crate bitflags;

pub trait ClickHouseSession {
    fn execute_query(&self, query: &str, stage: u64, writer: &mut ResultWriter) -> Result<()>;

    fn dbms_name(&self) -> &str {
        "clickhouse-server"
    }

    // None is by default, which will use same version as client send
    fn dbms_version_major(&self) -> u64 {
        19
    }

    fn dbms_version_minor(&self) -> u64 {
        17
    }

    fn dbms_tcp_protocol_version(&self) -> u64 {
        54428
    }

    fn timezone(&self) -> &str {
        "UTC"
    }

    fn server_display_name(&self) -> &str {
        "clickhouse-server"
    }

    fn dbms_version_patch(&self) -> u64 {
        1
    }
}

#[derive(Default)]
pub struct QueryState {
    pub query_id: String,
    pub stage: u64,
    pub compression: u64,
    pub query: String,
    pub is_cancelled: bool,
    pub is_connection_closed: bool,
    /// empty or not
    pub is_empty: bool,
    /// Data was sent.
    pub sent_all_data: bool,
}

impl QueryState {
    fn reset(&mut self) {
        self.stage = 0;
        self.is_cancelled = false;
        self.is_connection_closed = false;
        self.is_empty = false;
        self.sent_all_data = false;
    }
}

/// A server that speaks the ClickHouseprotocol, and can delegate client commands to a backend
/// that implements [`ClickHouseSession`]
pub struct ClickHouseServer<S, R: Read, W: Write> {
    session: S,
    reader: R,
    writer: W,

    query_state: QueryState,
    hello_request: HelloRequest,
}

impl<S: ClickHouseSession> ClickHouseServer<S, net::TcpStream, net::TcpStream> {
    pub fn run_on_tcp(session: S, stream: net::TcpStream) -> Result<()> {
        let w = stream.try_clone()?;
        ClickHouseServer::run_on(session, stream, w)
    }
}

impl<S: ClickHouseSession, ST: Read + Write + Clone> ClickHouseServer<S, ST, ST> {
    pub fn run_on_stream(session: S, stream: ST) -> Result<()> {
        ClickHouseServer::run_on(session, stream.clone(), stream)
    }
}

impl<S: ClickHouseSession, R: Read, W: Write> ClickHouseServer<S, R, W> {
    fn run_on(session: S, reader: R, writer: W) -> Result<()> {
        let mut srv = ClickHouseServer {
            session,
            reader,
            writer,
            query_state: Default::default(),
            hello_request: Default::default(),
        };

        srv.run()?;
        Ok(())
    }

    fn run(&mut self) -> Result<()> {
        debug!("Handle New session");

        self.process_hello()?;
        loop {
            // reset state
            self.query_state.reset();
            let packet_type_r: Result<u64> = self.reader.read_uvarint();
            let packet_type: u64;

            match packet_type_r {
                Err(e) => {
                    // TODO async io
                    if e.is_would_block() {
                        continue;
                    } else {
                        return Err(e);
                    }
                }
                Ok(v) => {
                    packet_type = v;
                }
            }

            debug!("Read packet_type {:?}", packet_type);

            match packet_type {
                protocols::CLIENT_PING => {
                    let mut encoder = Encoder::new();
                    encoder.uvarint(protocols::SERVER_PONG);
                    encoder.write_to(&mut self.writer)?;
                    continue;
                }
                protocols::CLIENT_CANCEL => continue,
                protocols::CLIENT_QUERY => self.process_query()?,
                protocols::CLIENT_DATA | protocols::CLIENT_SCALAR => {
                    self.process_data(packet_type == protocols::CLIENT_SCALAR)?
                }
                protocols::CLIENT_HELLO => {
                    let _ = self.receive_hello()?;
                    return Err("Unexpected packet Hello received from client"
                        .to_string()
                        .into());
                }

                _ => return Err(format!("Unhandle packet type:{}", packet_type).into()),
            }
        }

        debug!("Exited one session");
        Ok(())
    }

    fn receive_hello(&mut self) -> Result<HelloRequest> {
        let packet_type: u64 = self.reader.read_uvarint()?;
        if packet_type != protocols::SERVER_HELLO {
            // comes from http
            if packet_type == 'G' as u64 || packet_type == 'P' as u64 {
                let mut encoder = Encoder::new();
                encoder.string("HTTP/1.0 400 Bad Request\r\n\r\n");
                encoder.write_to(&mut self.writer)?;
                return Err("HTTP request wrong port, it's TCP port".into());
            }
        }
        HelloRequest::read_from(&mut self.reader)
    }

    fn process_hello(&mut self) -> Result<()> {
        let request = self.receive_hello()?;
        let response = HelloResponse {
            dbms_name: self.session.dbms_name().to_string(),
            dbms_version_major: self.session.dbms_version_major(),
            dbms_version_minor: self.session.dbms_version_minor(),
            dbms_tcp_protocol_version: self.session.dbms_tcp_protocol_version(),
            timezone: self.session.timezone().to_string(),
            server_display_name: self.session.server_display_name().to_string(),
            dbms_version_patch: self.session.dbms_version_patch(),
        };

        let mut encoder = Encoder::new();
        response.encode(&mut encoder, request.client_revision)?;
        encoder.write_to(&mut self.writer)?;

        self.hello_request = request;
        Ok(())
    }

    fn process_query(&mut self) -> Result<()> {
        let query_protocol =
            QueryProtocol::read_from(&mut self.reader, &self.hello_request, &mut self.query_state)?;

        let mut encoder = Encoder::new();

        self.query_state.query = query_protocol.query;
        self.query_state.stage = query_protocol.stage;
        self.query_state.compression = query_protocol.compression;

        let mut result_writer = ResultWriter::new(&mut encoder, self.query_state.compression > 0);

        if let Err(e) = self.session.execute_query(
            &self.query_state.query,
            self.query_state.stage,
            &mut result_writer,
        ) {
            ExceptionResponse::encode(&mut encoder, &e, true)?
        }

        encoder.uvarint(SERVER_END_OF_STREAM);
        encoder.write_to(&mut self.writer)?;
        Ok(())
    }

    fn process_data(&mut self, _scalar: bool) -> Result<()> {
        let _temporary_table = self.reader.read_string()?;
        let tz: Tz = self.session.timezone().parse()?;
        let _ = Block::load(&mut self.reader, tz, self.query_state.compression > 0)?;
        // TODO for insert
        Ok(())
    }
}

#[macro_export]
macro_rules! row {
    () => { $crate::types::RNil };
    ( $i:ident, $($tail:tt)* ) => {
        row!( $($tail)* ).put(stringify!($i).into(), $i.into())
    };
    ( $i:ident ) => { row!($i: $i) };

    ( $k:ident: $v:expr ) => {
        $crate::types::RNil.put(stringify!($k).into(), $v.into())
    };

    ( $k:ident: $v:expr, $($tail:tt)* ) => {
        row!( $($tail)* ).put(stringify!($k).into(), $v.into())
    };

    ( $k:expr => $v:expr ) => {
        $crate::types::RNil.put($k.into(), $v.into())
    };

    ( $k:expr => $v:expr, $($tail:tt)* ) => {
        row!( $($tail)* ).put($k.into(), $v.into())
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
