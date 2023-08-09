@echo off
set /p opt=Enter option code: 
@echo on
c:\rbrust\fcaacks-rs\target\debug\fcaacks-rs.exe %opt%
pause
