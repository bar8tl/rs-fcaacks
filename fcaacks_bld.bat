@echo off
cd c:\rbrust\fcaacks-rs\src\settings\envmnt
xcopy c:\c-portab\01-rb\pgmfiles\fcaacks.rs\crates\config.rs   . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\fcaacks.rs\crates\deflts.rs   . /D /C /Y
cd ..
xcopy c:\c-portab\01-rb\pgmfiles\fcaacks.rs\crates\params.rs   . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\fcaacks.rs\crates\envmnt.rs   . /D /C /Y
cd ..
xcopy c:\c-portab\01-rb\pgmfiles\fcaacks.rs\crates\settings.rs . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\fcaacks.rs\crates\create.rs   . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\fcaacks.rs\crates\inicopy.rs  . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\fcaacks.rs\crates\refresh.rs  . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\fcaacks.rs\crates\update.rs   . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\fcaacks.rs\crates\toexcel.rs  . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\fcaacks.rs\fcaacks.rs   main.rs /D /C /Y
cd ..
xcopy c:\c-portab\01-rb\pgmfiles\fcaacks.rs\Cargo.toml         . /D /C /Y
cargo build
pause
