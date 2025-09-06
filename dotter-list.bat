@echo off
REM List packages from dotter local.toml using sed

set LOCAL_TOML=.dotter\local.toml

if not exist "%LOCAL_TOML%" (
    echo Error: %LOCAL_TOML% not found
    exit /b 1
)

echo Current packages:
REM Extract package list and format nicely
for /f "delims=" %%i in ('sed -n "s/packages = \[\([^]]*\)\]/\1/p" "%LOCAL_TOML%" ^| sed "s/[^a-zA-Z0-9,]//g" ^| sed "s/,/ /g"') do (
    for %%j in (%%i) do (
        if not "%%j"=="" echo   - %%j
    )
)