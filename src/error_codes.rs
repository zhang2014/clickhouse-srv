bitflags! {
    pub struct ErrorCodes: u32{
        const UNSUPPORTED_METHOD= 1;
        const UNSUPPORTED_PARAMETER= 2;
        const UNEXPECTED_END_OF_FILE= 3;
        const EXPECTED_END_OF_FILE= 4;
        const CANNOT_PARSE_TEXT= 6;
        const INCORRECT_NUMBER_OF_COLUMNS= 7;
        const THERE_IS_NO_COLUMN= 8;
        const SIZES_OF_COLUMNS_DOESNT_MATCH= 9;
        const NOT_FOUND_COLUMN_IN_BLOCK= 10;
        const POSITION_OUT_OF_BOUND= 11;
        const PARAMETER_OUT_OF_BOUND= 12;
        const SIZES_OF_COLUMNS_IN_TUPLE_DOESNT_MATCH= 13;
        const DUPLICATE_COLUMN= 15;
        const NO_SUCH_COLUMN_IN_TABLE= 16;
        const DELIMITER_IN_STRING_LITERAL_DOESNT_MATCH= 17;
        const CANNOT_INSERT_ELEMENT_INTO_CONSTANT_COLUMN= 18;
        const SIZE_OF_FIXED_STRING_DOESNT_MATCH= 19;
        const NUMBER_OF_COLUMNS_DOESNT_MATCH= 20;
        const CANNOT_READ_ALL_DATA_FROM_TAB_SEPARATED_INPUT= 21;
        const CANNOT_PARSE_ALL_VALUE_FROM_TAB_SEPARATED_INPUT= 22;
        const CANNOT_READ_FROM_ISTREAM= 23;
        const CANNOT_WRITE_TO_OSTREAM= 24;
        const CANNOT_PARSE_ESCAPE_SEQUENCE= 25;
        const CANNOT_PARSE_QUOTED_STRING= 26;
        const CANNOT_PARSE_INPUT_ASSERTION_FAILED= 27;
        const CANNOT_PRINT_FLOAT_OR_DOUBLE_NUMBER= 28;
        const CANNOT_PRINT_INTEGER= 29;
        const CANNOT_READ_SIZE_OF_COMPRESSED_CHUNK= 30;
        const CANNOT_READ_COMPRESSED_CHUNK= 31;
        const ATTEMPT_TO_READ_AFTER_EOF= 32;
        const CANNOT_READ_ALL_DATA= 33;
        const TOO_MANY_ARGUMENTS_FOR_FUNCTION= 34;
        const TOO_FEW_ARGUMENTS_FOR_FUNCTION= 35;
        const BAD_ARGUMENTS= 36;
        const UNKNOWN_ELEMENT_IN_AST= 37;
        const CANNOT_PARSE_DATE= 38;
        const TOO_LARGE_SIZE_COMPRESSED= 39;
        const CHECKSUM_DOESNT_MATCH= 40;
        const CANNOT_PARSE_DATETIME= 41;
        const NUMBER_OF_ARGUMENTS_DOESNT_MATCH= 42;
        const ILLEGAL_TYPE_OF_ARGUMENT= 43;
        const ILLEGAL_COLUMN= 44;
        const ILLEGAL_NUMBER_OF_RESULT_COLUMNS= 45;
        const UNKNOWN_FUNCTION= 46;
        const UNKNOWN_IDENTIFIER= 47;
        const NOT_IMPLEMENTED= 48;
        const LOGICAL_ERROR= 49;
        const UNKNOWN_TYPE= 50;
        const EMPTY_LIST_OF_COLUMNS_QUERIED= 51;
        const COLUMN_QUERIED_MORE_THAN_ONCE= 52;
        const TYPE_MISMATCH= 53;
        const STORAGE_DOESNT_ALLOW_PARAMETERS= 54;
        const STORAGE_REQUIRES_PARAMETER= 55;
        const UNKNOWN_STORAGE= 56;
        const TABLE_ALREADY_EXISTS= 57;
        const TABLE_METADATA_ALREADY_EXISTS= 58;
        const ILLEGAL_TYPE_OF_COLUMN_FOR_FILTER= 59;
        const UNKNOWN_TABLE= 60;
        const ONLY_FILTER_COLUMN_IN_BLOCK= 61;
        const SYNTAX_ERROR= 62;
        const UNKNOWN_AGGREGATE_FUNCTION= 63;
        const CANNOT_READ_AGGREGATE_FUNCTION_FROM_TEXT= 64;
        const CANNOT_WRITE_AGGREGATE_FUNCTION_AS_TEXT= 65;
        const NOT_A_COLUMN= 66;
        const ILLEGAL_KEY_OF_AGGREGATION= 67;
        const CANNOT_GET_SIZE_OF_FIELD= 68;
        const ARGUMENT_OUT_OF_BOUND= 69;
        const CANNOT_CONVERT_TYPE= 70;
        const CANNOT_WRITE_AFTER_END_OF_BUFFER= 71;
        const CANNOT_PARSE_NUMBER= 72;
        const UNKNOWN_FORMAT= 73;
        const CANNOT_READ_FROM_FILE_DESCRIPTOR= 74;
        const CANNOT_WRITE_TO_FILE_DESCRIPTOR= 75;
        const CANNOT_OPEN_FILE= 76;
        const CANNOT_CLOSE_FILE= 77;
        const UNKNOWN_TYPE_OF_QUERY= 78;
        const INCORRECT_FILE_NAME= 79;
        const INCORRECT_QUERY= 80;
        const UNKNOWN_DATABASE= 81;
        const DATABASE_ALREADY_EXISTS= 82;
        const DIRECTORY_DOESNT_EXIST= 83;
        const DIRECTORY_ALREADY_EXISTS= 84;
        const FORMAT_IS_NOT_SUITABLE_FOR_INPUT= 85;
        const RECEIVED_ERROR_FROM_REMOTE_IO_SERVER= 86;
        const CANNOT_SEEK_THROUGH_FILE= 87;
        const CANNOT_TRUNCATE_FILE= 88;
        const UNKNOWN_COMPRESSION_METHOD= 89;
        const EMPTY_LIST_OF_COLUMNS_PASSED= 90;
        const SIZES_OF_MARKS_FILES_ARE_INCONSISTENT= 91;
        const EMPTY_DATA_PASSED= 92;
        const UNKNOWN_AGGREGATED_DATA_VARIANT= 93;
        const CANNOT_MERGE_DIFFERENT_AGGREGATED_DATA_VARIANTS= 94;
        const CANNOT_READ_FROM_SOCKET= 95;
        const CANNOT_WRITE_TO_SOCKET= 96;
        const CANNOT_READ_ALL_DATA_FROM_CHUNKED_INPUT= 97;
        const CANNOT_WRITE_TO_EMPTY_BLOCK_OUTPUT_STREAM= 98;
        const UNKNOWN_PACKET_FROM_CLIENT= 99;
        const UNKNOWN_PACKET_FROM_SERVER= 100;
        const UNEXPECTED_PACKET_FROM_CLIENT= 101;
        const UNEXPECTED_PACKET_FROM_SERVER= 102;
        const RECEIVED_DATA_FOR_WRONG_QUERY_ID= 103;
        const TOO_SMALL_BUFFER_SIZE= 104;
        const CANNOT_READ_HISTORY= 105;
        const CANNOT_APPEND_HISTORY= 106;
        const FILE_DOESNT_EXIST= 107;
        const NO_DATA_TO_INSERT= 108;
        const CANNOT_BLOCK_SIGNAL= 109;
        const CANNOT_UNBLOCK_SIGNAL= 110;
        const CANNOT_MANIPULATE_SIGSET= 111;
        const CANNOT_WAIT_FOR_SIGNAL= 112;
        const THERE_IS_NO_SESSION= 113;
        const CANNOT_CLOCK_GETTIME= 114;
        const UNKNOWN_SETTING= 115;
        const THERE_IS_NO_DEFAULT_VALUE= 116;
        const INCORRECT_DATA= 117;
        const ENGINE_REQUIRED= 119;
        const CANNOT_INSERT_VALUE_OF_DIFFERENT_SIZE_INTO_TUPLE= 120;
        const UNSUPPORTED_JOIN_KEYS= 121;
        const INCOMPATIBLE_COLUMNS= 122;
        const UNKNOWN_TYPE_OF_AST_NODE= 123;
        const INCORRECT_ELEMENT_OF_SET= 124;
        const INCORRECT_RESULT_OF_SCALAR_SUBQUERY= 125;
        const CANNOT_GET_RETURN_TYPE= 126;
        const ILLEGAL_INDEX= 127;
        const TOO_LARGE_ARRAY_SIZE= 128;
        const FUNCTION_IS_SPECIAL= 129;
        const CANNOT_READ_ARRAY_FROM_TEXT= 130;
        const TOO_LARGE_STRING_SIZE= 131;
        const CANNOT_CREATE_TABLE_FROM_METADATA= 132;
        const AGGREGATE_FUNCTION_DOESNT_ALLOW_PARAMETERS= 133;
        const PARAMETERS_TO_AGGREGATE_FUNCTIONS_MUST_BE_LITERALS= 134;
        const ZERO_ARRAY_OR_TUPLE_INDEX= 135;
        const UNKNOWN_ELEMENT_IN_CONFIG= 137;
        const EXCESSIVE_ELEMENT_IN_CONFIG= 138;
        const NO_ELEMENTS_IN_CONFIG= 139;
        const ALL_REQUESTED_COLUMNS_ARE_MISSING= 140;
        const SAMPLING_NOT_SUPPORTED= 141;
        const NOT_FOUND_NODE= 142;
        const FOUND_MORE_THAN_ONE_NODE= 143;
        const FIRST_DATE_IS_BIGGER_THAN_LAST_DATE= 144;
        const UNKNOWN_OVERFLOW_MODE= 145;
        const QUERY_SECTION_DOESNT_MAKE_SENSE= 146;
        const NOT_FOUND_FUNCTION_ELEMENT_FOR_AGGREGATE= 147;
        const NOT_FOUND_RELATION_ELEMENT_FOR_CONDITION= 148;
        const NOT_FOUND_RHS_ELEMENT_FOR_CONDITION= 149;
        const EMPTY_LIST_OF_ATTRIBUTES_PASSED= 150;
        const INDEX_OF_COLUMN_IN_SORT_CLAUSE_IS_OUT_OF_RANGE= 151;
        const UNKNOWN_DIRECTION_OF_SORTING= 152;
        const ILLEGAL_DIVISION= 153;
        const AGGREGATE_FUNCTION_NOT_APPLICABLE= 154;
        const UNKNOWN_RELATION= 155;
        const DICTIONARIES_WAS_NOT_LOADED= 156;
        const ILLEGAL_OVERFLOW_MODE= 157;
        const TOO_MANY_ROWS= 158;
        const TIMEOUT_EXCEEDED= 159;
        const TOO_SLOW= 160;
        const TOO_MANY_COLUMNS= 161;
        const TOO_DEEP_SUBQUERIES= 162;
        const TOO_DEEP_PIPELINE= 163;
        const READONLY= 164;
        const TOO_MANY_TEMPORARY_COLUMNS= 165;
        const TOO_MANY_TEMPORARY_NON_CONST_COLUMNS= 166;
        const TOO_DEEP_AST= 167;
        const TOO_BIG_AST= 168;
        const BAD_TYPE_OF_FIELD= 169;
        const BAD_GET= 170;
        const BLOCKS_HAVE_DIFFERENT_STRUCTURE= 171;
        const CANNOT_CREATE_DIRECTORY= 172;
        const CANNOT_ALLOCATE_MEMORY= 173;
        const CYCLIC_ALIASES= 174;
        const CHUNK_NOT_FOUND= 176;
        const DUPLICATE_CHUNK_NAME= 177;
        const MULTIPLE_ALIASES_FOR_EXPRESSION= 178;
        const MULTIPLE_EXPRESSIONS_FOR_ALIAS= 179;
        const THERE_IS_NO_PROFILE= 180;
        const ILLEGAL_FINAL= 181;
        const ILLEGAL_PREWHERE= 182;
        const UNEXPECTED_EXPRESSION= 183;
        const ILLEGAL_AGGREGATION= 184;
        const UNSUPPORTED_MYISAM_BLOCK_TYPE= 185;
        const UNSUPPORTED_COLLATION_LOCALE= 186;
        const COLLATION_COMPARISON_FAILED= 187;
        const UNKNOWN_ACTION= 188;
        const TABLE_MUST_NOT_BE_CREATED_MANUALLY= 189;
        const SIZES_OF_ARRAYS_DOESNT_MATCH= 190;
        const SET_SIZE_LIMIT_EXCEEDED= 191;
        const UNKNOWN_USER= 192;
        const WRONG_PASSWORD= 193;
        const REQUIRED_PASSWORD= 194;
        const IP_ADDRESS_NOT_ALLOWED= 195;
        const UNKNOWN_ADDRESS_PATTERN_TYPE= 196;
        const SERVER_REVISION_IS_TOO_OLD= 197;
        const DNS_ERROR= 198;
        const UNKNOWN_QUOTA= 199;
        const QUOTA_DOESNT_ALLOW_KEYS= 200;
        const QUOTA_EXPIRED= 201;
        const TOO_MANY_SIMULTANEOUS_QUERIES= 202;
        const NO_FREE_CONNECTION= 203;
        const CANNOT_FSYNC= 204;
        const NESTED_TYPE_TOO_DEEP= 205;
        const ALIAS_REQUIRED= 206;
        const AMBIGUOUS_IDENTIFIER= 207;
        const EMPTY_NESTED_TABLE= 208;
        const SOCKET_TIMEOUT= 209;
        const NETWORK_ERROR= 210;
        const EMPTY_QUERY= 211;
        const UNKNOWN_LOAD_BALANCING= 212;
        const UNKNOWN_TOTALS_MODE= 213;
        const CANNOT_STATVFS= 214;
        const NOT_AN_AGGREGATE= 215;
        const QUERY_WITH_SAME_ID_IS_ALREADY_RUNNING= 216;
        const CLIENT_HAS_CONNECTED_TO_WRONG_PORT= 217;
        const TABLE_IS_DROPPED= 218;
        const DATABASE_NOT_EMPTY= 219;
        const DUPLICATE_INTERSERVER_IO_ENDPOINT= 220;
        const NO_SUCH_INTERSERVER_IO_ENDPOINT= 221;
        const ADDING_REPLICA_TO_NON_EMPTY_TABLE= 222;
        const UNEXPECTED_AST_STRUCTURE= 223;
        const REPLICA_IS_ALREADY_ACTIVE= 224;
        const NO_ZOOKEEPER= 225;
        const NO_FILE_IN_DATA_PART= 226;
        const UNEXPECTED_FILE_IN_DATA_PART= 227;
        const BAD_SIZE_OF_FILE_IN_DATA_PART= 228;
        const QUERY_IS_TOO_LARGE= 229;
        const NOT_FOUND_EXPECTED_DATA_PART= 230;
        const TOO_MANY_UNEXPECTED_DATA_PARTS= 231;
        const NO_SUCH_DATA_PART= 232;
        const BAD_DATA_PART_NAME= 233;
        const NO_REPLICA_HAS_PART= 234;
        const DUPLICATE_DATA_PART= 235;
        const ABORTED= 236;
        const NO_REPLICA_NAME_GIVEN= 237;
        const FORMAT_VERSION_TOO_OLD= 238;
        const CANNOT_MUNMAP= 239;
        const CANNOT_MREMAP= 240;
        const MEMORY_LIMIT_EXCEEDED= 241;
        const TABLE_IS_READ_ONLY= 242;
        const NOT_ENOUGH_SPACE= 243;
        const UNEXPECTED_ZOOKEEPER_ERROR= 244;
        const CORRUPTED_DATA= 246;
        const INCORRECT_MARK= 247;
        const INVALID_PARTITION_VALUE= 248;
        const NOT_ENOUGH_BLOCK_NUMBERS= 250;
        const NO_SUCH_REPLICA= 251;
        const TOO_MANY_PARTS= 252;
        const REPLICA_IS_ALREADY_EXIST= 253;
        const NO_ACTIVE_REPLICAS= 254;
        const TOO_MANY_RETRIES_TO_FETCH_PARTS= 255;
        const PARTITION_ALREADY_EXISTS= 256;
        const PARTITION_DOESNT_EXIST= 257;
        const UNION_ALL_RESULT_STRUCTURES_MISMATCH= 258;
        const CLIENT_OUTPUT_FORMAT_SPECIFIED= 260;
        const UNKNOWN_BLOCK_INFO_FIELD= 261;
        const BAD_COLLATION= 262;
        const CANNOT_COMPILE_CODE= 263;
        const INCOMPATIBLE_TYPE_OF_JOIN= 264;
        const NO_AVAILABLE_REPLICA= 265;
        const MISMATCH_REPLICAS_DATA_SOURCES= 266;
        const STORAGE_DOESNT_SUPPORT_PARALLEL_REPLICAS= 267;
        const CPUID_ERROR= 268;
        const INFINITE_LOOP= 269;
        const CANNOT_COMPRESS= 270;
        const CANNOT_DECOMPRESS= 271;
        const CANNOT_IO_SUBMIT= 272;
        const CANNOT_IO_GETEVENTS= 273;
        const AIO_READ_ERROR= 274;
        const AIO_WRITE_ERROR= 275;
        const INDEX_NOT_USED= 277;
        const LEADERSHIP_LOST= 278;
        const ALL_CONNECTION_TRIES_FAILED= 279;
        const NO_AVAILABLE_DATA= 280;
        const DICTIONARY_IS_EMPTY= 281;
        const INCORRECT_INDEX= 282;
        const UNKNOWN_DISTRIBUTED_PRODUCT_MODE= 283;
        const UNKNOWN_GLOBAL_SUBQUERIES_METHOD= 284;
        const TOO_FEW_LIVE_REPLICAS= 285;
        const UNSATISFIED_QUORUM_FOR_PREVIOUS_WRITE= 286;
        const UNKNOWN_FORMAT_VERSION= 287;
        const DISTRIBUTED_IN_JOIN_SUBQUERY_DENIED= 288;
        const REPLICA_IS_NOT_IN_QUORUM= 289;
        const LIMIT_EXCEEDED= 290;
        const DATABASE_ACCESS_DENIED= 291;
        const LEADERSHIP_CHANGED= 292;
        const MONGODB_CANNOT_AUTHENTICATE= 293;
        const INVALID_BLOCK_EXTRA_INFO= 294;
        const RECEIVED_EMPTY_DATA= 295;
        const NO_REMOTE_SHARD_FOUND= 296;
        const SHARD_HAS_NO_CONNECTIONS= 297;
        const CANNOT_PIPE= 298;
        const CANNOT_FORK= 299;
        const CANNOT_DLSYM= 300;
        const CANNOT_CREATE_CHILD_PROCESS= 301;
        const CHILD_WAS_NOT_EXITED_NORMALLY= 302;
        const CANNOT_SELECT= 303;
        const CANNOT_WAITPID= 304;
        const TABLE_WAS_NOT_DROPPED= 305;
        const TOO_DEEP_RECURSION= 306;
        const TOO_MANY_BYTES= 307;
        const UNEXPECTED_NODE_IN_ZOOKEEPER= 308;
        const FUNCTION_CANNOT_HAVE_PARAMETERS= 309;
        const INVALID_SHARD_WEIGHT= 317;
        const INVALID_CONFIG_PARAMETER= 318;
        const UNKNOWN_STATUS_OF_INSERT= 319;
        const VALUE_IS_OUT_OF_RANGE_OF_DATA_TYPE= 321;
        const BARRIER_TIMEOUT= 335;
        const UNKNOWN_DATABASE_ENGINE= 336;
        const DDL_GUARD_IS_ACTIVE= 337;
        const UNFINISHED= 341;
        const METADATA_MISMATCH= 342;
        const SUPPORT_IS_DISABLED= 344;
        const TABLE_DIFFERS_TOO_MUCH= 345;
        const CANNOT_CONVERT_CHARSET= 346;
        const CANNOT_LOAD_CONFIG= 347;
        const CANNOT_INSERT_NULL_IN_ORDINARY_COLUMN= 349;
        const INCOMPATIBLE_SOURCE_TABLES= 350;
        const AMBIGUOUS_TABLE_NAME= 351;
        const AMBIGUOUS_COLUMN_NAME= 352;
        const INDEX_OF_POSITIONAL_ARGUMENT_IS_OUT_OF_RANGE= 353;
        const ZLIB_INFLATE_FAILED= 354;
        const ZLIB_DEFLATE_FAILED= 355;
        const BAD_LAMBDA= 356;
        const RESERVED_IDENTIFIER_NAME= 357;
        const INTO_OUTFILE_NOT_ALLOWED= 358;
        const TABLE_SIZE_EXCEEDS_MAX_DROP_SIZE_LIMIT= 359;
        const CANNOT_CREATE_CHARSET_CONVERTER= 360;
        const SEEK_POSITION_OUT_OF_BOUND= 361;
        const CURRENT_WRITE_BUFFER_IS_EXHAUSTED= 362;
        const CANNOT_CREATE_IO_BUFFER= 363;
        const RECEIVED_ERROR_TOO_MANY_REQUESTS= 364;
        const OUTPUT_IS_NOT_SORTED= 365;
        const SIZES_OF_NESTED_COLUMNS_ARE_INCONSISTENT= 366;
        const TOO_MANY_FETCHES= 367;
        const BAD_CAST= 368;
        const ALL_REPLICAS_ARE_STALE= 369;
        const DATA_TYPE_CANNOT_BE_USED_IN_TABLES= 370;
        const INCONSISTENT_CLUSTER_DEFINITION= 371;
        const SESSION_NOT_FOUND= 372;
        const SESSION_IS_LOCKED= 373;
        const INVALID_SESSION_TIMEOUT= 374;
        const CANNOT_DLOPEN= 375;
        const CANNOT_PARSE_UUID= 376;
        const ILLEGAL_SYNTAX_FOR_DATA_TYPE= 377;
        const DATA_TYPE_CANNOT_HAVE_ARGUMENTS= 378;
        const UNKNOWN_STATUS_OF_DISTRIBUTED_DDL_TASK= 379;
        const CANNOT_KILL= 380;
        const HTTP_LENGTH_REQUIRED= 381;
        const CANNOT_LOAD_CATBOOST_MODEL= 382;
        const CANNOT_APPLY_CATBOOST_MODEL= 383;
        const PART_IS_TEMPORARILY_LOCKED= 384;
        const MULTIPLE_STREAMS_REQUIRED= 385;
        const NO_COMMON_TYPE= 386;
        const DICTIONARY_ALREADY_EXISTS= 387;
        const CANNOT_ASSIGN_OPTIMIZE= 388;
        const INSERT_WAS_DEDUPLICATED= 389;
        const CANNOT_GET_CREATE_TABLE_QUERY= 390;
        const EXTERNAL_LIBRARY_ERROR= 391;
        const QUERY_IS_PROHIBITED= 392;
        const THERE_IS_NO_QUERY= 393;
        const QUERY_WAS_CANCELLED= 394;
        const FUNCTION_THROW_IF_VALUE_IS_NON_ZERO= 395;
        const TOO_MANY_ROWS_OR_BYTES= 396;
        const QUERY_IS_NOT_SUPPORTED_IN_MATERIALIZED_VIEW= 397;
        const UNKNOWN_MUTATION_COMMAND= 398;
        const FORMAT_IS_NOT_SUITABLE_FOR_OUTPUT= 399;
        const CANNOT_STAT= 400;
        const FEATURE_IS_NOT_ENABLED_AT_BUILD_TIME= 401;
        const CANNOT_IOSETUP= 402;
        const INVALID_JOIN_ON_EXPRESSION= 403;
        const BAD_ODBC_CONNECTION_STRING= 404;
        const PARTITION_SIZE_EXCEEDS_MAX_DROP_SIZE_LIMIT= 405;
        const TOP_AND_LIMIT_TOGETHER= 406;
        const DECIMAL_OVERFLOW= 407;
        const BAD_REQUEST_PARAMETER= 408;
        const EXTERNAL_EXECUTABLE_NOT_FOUND= 409;
        const EXTERNAL_SERVER_IS_NOT_RESPONDING= 410;
        const PTHREAD_ERROR= 411;
        const NETLINK_ERROR= 412;
        const CANNOT_SET_SIGNAL_HANDLER= 413;
        const CANNOT_READLINE= 414;
        const ALL_REPLICAS_LOST= 415;
        const REPLICA_STATUS_CHANGED= 416;
        const EXPECTED_ALL_OR_ANY= 417;
        const UNKNOWN_JOIN_STRICTNESS= 418;
        const MULTIPLE_ASSIGNMENTS_TO_COLUMN= 419;
        const CANNOT_UPDATE_COLUMN= 420;
        const CANNOT_ADD_DIFFERENT_AGGREGATE_STATES= 421;
        const UNSUPPORTED_URI_SCHEME= 422;
        const CANNOT_GETTIMEOFDAY= 423;
        const CANNOT_LINK= 424;
        const SYSTEM_ERROR= 425;
        const NULL_POINTER_DEREFERENCE= 426;
        const CANNOT_COMPILE_REGEXP= 427;
        const UNKNOWN_LOG_LEVEL= 428;
        const FAILED_TO_GETPWUID= 429;
        const MISMATCHING_USERS_FOR_PROCESS_AND_DATA= 430;
        const ILLEGAL_SYNTAX_FOR_CODEC_TYPE= 431;
        const UNKNOWN_CODEC= 432;
        const ILLEGAL_CODEC_PARAMETER= 433;
        const CANNOT_PARSE_PROTOBUF_SCHEMA= 434;
        const NO_DATA_FOR_REQUIRED_PROTOBUF_FIELD= 435;
        const PROTOBUF_BAD_CAST= 436;
        const PROTOBUF_FIELD_NOT_REPEATED= 437;
        const DATA_TYPE_CANNOT_BE_PROMOTED= 438;
        const CANNOT_SCHEDULE_TASK= 439;
        const INVALID_LIMIT_EXPRESSION= 440;
        const CANNOT_PARSE_DOMAIN_VALUE_FROM_STRING= 441;
        const BAD_DATABASE_FOR_TEMPORARY_TABLE= 442;
        const NO_COMMON_COLUMNS_WITH_PROTOBUF_SCHEMA= 443;
        const UNKNOWN_PROTOBUF_FORMAT= 444;
        const CANNOT_MPROTECT= 445;
        const FUNCTION_NOT_ALLOWED= 446;
        const HYPERSCAN_CANNOT_SCAN_TEXT= 447;
        const BROTLI_READ_FAILED= 448;
        const BROTLI_WRITE_FAILED= 449;
        const BAD_TTL_EXPRESSION= 450;
        const BAD_TTL_FILE= 451;
        const SETTING_CONSTRAINT_VIOLATION= 452;
        const MYSQL_CLIENT_INSUFFICIENT_CAPABILITIES= 453;
        const OPENSSL_ERROR= 454;
        const SUSPICIOUS_TYPE_FOR_LOW_CARDINALITY= 455;
        const UNKNOWN_QUERY_PARAMETER= 456;
        const BAD_QUERY_PARAMETER= 457;
        const CANNOT_UNLINK= 458;
        const CANNOT_SET_THREAD_PRIORITY= 459;
        const CANNOT_CREATE_TIMER= 460;
        const CANNOT_SET_TIMER_PERIOD= 461;
        const CANNOT_DELETE_TIMER= 462;
        const CANNOT_FCNTL= 463;
        const CANNOT_PARSE_ELF= 464;
        const CANNOT_PARSE_DWARF= 465;
        const INSECURE_PATH= 466;
        const CANNOT_PARSE_BOOL= 467;
        const CANNOT_PTHREAD_ATTR= 468;
        const VIOLATED_CONSTRAINT= 469;
        const QUERY_IS_NOT_SUPPORTED_IN_LIVE_VIEW= 470;
        const SETTINGS_ARE_NOT_SUPPORTED= 471;
        const READONLY_SETTING= 472;
        const DEADLOCK_AVOIDED= 473;
        const INVALID_TEMPLATE_FORMAT= 474;
        const INVALID_WITH_FILL_EXPRESSION= 475;
        const WITH_TIES_WITHOUT_ORDER_BY= 476;
        const INVALID_USAGE_OF_INPUT= 477;
        const UNKNOWN_POLICY= 478;
        const UNKNOWN_DISK= 479;
        const UNKNOWN_PROTOCOL= 480;
        const PATH_ACCESS_DENIED= 481;
        const DICTIONARY_ACCESS_DENIED= 482;
        const TOO_MANY_REDIRECTS= 483;
        const INTERNAL_REDIS_ERROR= 484;
        const SCALAR_ALREADY_EXISTS= 485;
        const UNKNOWN_SCALAR= 486;
        const CANNOT_GET_CREATE_DICTIONARY_QUERY= 487;
        const UNKNOWN_DICTIONARY= 488;
        const INCORRECT_DICTIONARY_DEFINITION= 489;
        const CANNOT_FORMAT_DATETIME= 490;
        const UNACCEPTABLE_URL= 491;
        const ACCESS_ENTITY_NOT_FOUND= 492;
        const ACCESS_ENTITY_ALREADY_EXISTS= 493;
        const ACCESS_ENTITY_FOUND_DUPLICATES= 494;
        const ACCESS_ENTITY_STORAGE_READONLY= 495;
        const QUOTA_REQUIRES_CLIENT_KEY= 496;
        const NOT_ENOUGH_PRIVILEGES= 497;
        const LIMIT_BY_WITH_TIES_IS_NOT_SUPPORTED= 498;
        const S3_ERROR= 499;
        const CANNOT_CREATE_DICTIONARY_FROM_METADATA= 500;
        const CANNOT_CREATE_DATABASE= 501;
        const KEEPER_EXCEPTION= 999;
        const POCO_EXCEPTION= 1000;
        const STD_EXCEPTION= 1001;
        const UNKNOWN_EXCEPTION= 1002;
        const CONDITIONAL_TREE_PARENT_NOT_FOUND= 2001;
        const ILLEGAL_PROJECTION_MANIPULATOR= 2002;
    }
}
