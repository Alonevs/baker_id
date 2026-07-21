@echo off
echo ========================================
echo   Forge Editor 2D
echo ========================================
echo.

REM Cambiar al directorio del proyecto
cd /d "%~dp0"

REM Verificar si cargo está disponible
where cargo >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo Error: Cargo no encontrado
    echo Asegúrate de tener Rust instalado
    pause
    exit /b 1
)

echo Starting Forge Editor 2D...
echo.

REM Ejecutar el editor desde el binario compilado
start "Forge Editor 2D" target\release\forge.exe
timeout /t 2 >nul
echo.
echo Editor lanzado. Presiona una tecla para cerrar esta ventana.
pause