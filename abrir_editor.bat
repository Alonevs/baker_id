@echo off
chcp 65001 >nul
title Forge Editor 2D

cd /d "%~dp0"

echo ========================================================
echo   🔥 Iniciando Forge Editor 2D (Modo Release)...
echo ========================================================
echo.

cargo run -p forge-editor --release

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [!] Hubo un problema al ejecutar Forge Editor.
    pause
)
