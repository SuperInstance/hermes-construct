@echo off
echo Moving eveng1_python_sdk out of System32...
robocopy "C:\Windows\System32\eveng1_python_sdk" "C:\Users\casey\eveng1_python_sdk" /E /COPYALL /R:0 /W:0
echo.
echo Attempting to remove source folder...
rd /S /Q "C:\Windows\System32\eveng1_python_sdk"
if exist "C:\Windows\System32\eveng1_python_sdk" (
    echo Source still in use ? scheduling removal at next reboot...
    reg add "HKLM\SYSTEM\CurrentControlSet\Control\Session Manager" /v PendingFileRenameOperations /t REG_MULTI_SZ /d "\\??\\C:\\Windows\\System32\\eveng1_python_sdk\\* " /f
)
echo.
echo Done. If scheduled, please reboot to complete.
pause
