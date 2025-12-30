@echo off
cargo build --release
if %ERRORLEVEL% EQU 0 (
    mkdir build
    copy /Y target\release\driver.dll build\driver.sys
    echo Successfully created driver.sys
) else (
    echo Build failed
)
pause
