@echo off
REM Remove package from dotter local.toml using sed

if "%1"=="" (
    echo Usage: %0 ^<package-name^>
    exit /b 1
)

set PACKAGE=%1
set LOCAL_TOML=.dotter\local.toml

if not exist "%LOCAL_TOML%" (
    echo Error: %LOCAL_TOML% not found
    exit /b 1
)

REM Check if package exists
findstr /c:"\"%PACKAGE%\"" "%LOCAL_TOML%" >nul
if not %errorlevel%==0 (
    echo Package '%PACKAGE%' not found
    exit /b 0
)

REM Remove package from the list
sed -i "s/, \"%PACKAGE%\"//g; s/\"%PACKAGE%\", //g; s/\"%PACKAGE%\"//g" "%LOCAL_TOML%"

echo Removed package '%PACKAGE%' from local.toml
echo Run 'dotter deploy' to apply changes