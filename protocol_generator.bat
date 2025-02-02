@echo off
flatc --gen-all --gen-onefile --rust -o schema/generated schema/src/protocol.fbs
flatc --gen-all --gen-onefile --csharp -o schema/generated schema/src/protocol.fbs

set SOURCE_PATH=schema\generated\protocol_generated.rs
set DEST_DIR=mmp_prototype\src\
copy "%SOURCE_PATH%" "%DEST_DIR%"