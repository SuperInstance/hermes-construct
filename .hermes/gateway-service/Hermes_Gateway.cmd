@echo off
rem Hermes Agent Gateway - Messaging Platform Integration
cd /d C:\Users\casey\hermes-construct\hermes-construct\venv\Lib\site-packages
set "HERMES_HOME=C:\Users\casey\.hermes"
set "PYTHONIOENCODING=utf-8"
set "HERMES_GATEWAY_DETACHED=1"
set "VIRTUAL_ENV=C:\Users\casey\hermes-construct\hermes-construct\venv"
C:\Users\casey\hermes-construct\hermes-construct\venv\Scripts\pythonw.exe -m hermes_cli.main gateway run
exit /b 0
