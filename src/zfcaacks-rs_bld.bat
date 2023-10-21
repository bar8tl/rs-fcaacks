@echo off
rem zfcaacks-rs_bld.bat - Script to start compiling of program fcaacks-rs
rem (2019-03-01 bar8tl)
xcopy _config.json ..\target\debug\ /D /C /Y
cd ..\
cargo build
pause
