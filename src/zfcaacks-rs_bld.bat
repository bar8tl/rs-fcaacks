@echo off
xcopy fcaacks-cfg.json ..\target\debug\ /D /C /Y
cd ..\
cargo build
pause
