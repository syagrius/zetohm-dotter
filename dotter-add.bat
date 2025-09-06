@echo off
REM Add package to dotter local.toml using sed

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

REM Check if package already exists
findstr /c:"\"%PACKAGE%\"" "%LOCAL_TOML%" >nul
if %errorlevel%==0 (
    echo Package '%PACKAGE%' already exists
    exit /b 0
)

REM Add package to the list
sed -i "s/packages = \[\([^]]*\)\]/packages = [\1, \"%PACKAGE%\"]/" "%LOCAL_TOML%"

REM Clean up formatting (remove leading comma and space)
sed -i "s/packages = \[, /packages = [/" "%LOCAL_TOML%"

echo Added package '%PACKAGE%' to local.toml
echo Run 'dotter deploy' to apply changes