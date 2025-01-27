@echo off
flatc --gen-all --rust -o schema/generated schema/src/protocol.fbs

set SOURCE_PATH=schema\generated\protocol_generated.rs
set DEST_DIR=alphaserver\src\
copy "%SOURCE_PATH%" "%DEST_DIR%"

set DEST_DIR=chatclient\src\
copy "%SOURCE_PATH%" "%DEST_DIR%"