@echo off
echo Starting ActiveLog.ai...
wsl -d Ubuntu-22.04 -u root -e bash -c "su - activeloguser -c '~/activelog/start_activelog.sh'"
timeout /t 3
start http://localhost:3000
start http://localhost:8000/docs