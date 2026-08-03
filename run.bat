@echo off
chcp 65001 >nul
title Forge Editor 2D Launcher

echo ========================================================
echo                 🔥 FORGE EDITOR 2D 🔥                   
echo ========================================================
echo.

REM Cambiar al directorio del script
cd /d "%~dp0"

REM Verificar si Cargo está instalado
where cargo >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Cargo no está instalado o no se encuentra en el PATH.
    echo Por favor, instala Rust desde https://rustup.rs/
    echo.
    pause
    exit /b 1
)

echo Selecciona el modo de ejecución:
echo.
echo  [1] Modo Optimizado (Release) - Recomendado para máximo rendimiento
echo  [2] Modo Desarrollo (Debug)   - Compilación más rápida
echo  [3] Ejecutar Tests            - Verifica la suite de pruebas
echo.
set /p opcion="Elige una opción (1-3) [Por defecto: 1]: "

if "%opcion%"=="2" goto run_debug
if "%opcion%"=="3" goto run_tests
goto run_release

:run_release
echo.
echo 🚀 Iniciando Forge Editor 2D en Modo Release...
echo.
cargo run -p forge-editor --release
goto end

:run_debug
echo.
echo ⚡ Iniciando Forge Editor 2D en Modo Debug...
echo.
cargo run -p forge-editor
goto end

:run_tests
echo.
echo 🧪 Ejecutando suite de pruebas del workspace...
echo.
cargo test --workspace
goto end

:end
echo.
echo Editor o proceso finalizado. Presiona una tecla para salir.
pause