Write-Host "=== Try to see diskpart window title via Windows API ==="
Add-Type @"
using System;
using System.Runtime.InteropServices;
using System.Text;
public class WinHelper {
    [DllImport("user32.dll")]
    public static extern IntPtr GetForegroundWindow();
    [DllImport("user32.dll")]
    public static extern int GetWindowText(IntPtr hWnd, StringBuilder text, int count);
}
"@

$hwnd = [WinHelper]::GetForegroundWindow()
$sb = New-Object System.Text.StringBuilder 256
[void][WinHelper]::GetWindowText($hwnd, $sb, 256)
Write-Host "Foreground window: $($sb.ToString())"

Write-Host "`n=== List top-level windows for diskpart ==="
Get-Process diskpart -ErrorAction SilentlyContinue | ForEach-Object {
    Write-Host "PID $($_.Id) - MainWindowTitle: '$($_.MainWindowTitle)' - MainWindowHandle: $($_.MainWindowHandle)"
}

Write-Host "`n=== Disk state ==="
Write-Host ("Free: {0} GB" -f [math]::Round((Get-PSDrive C).Free/1GB,2))
$v = Get-Item "$env:LOCALAPPDATA\wsl\{ce718fe0-2cfd-4ad9-9995-20b13b2f2ff9}\ext4.vhdx"
Write-Host ("vhdx: {0} GB" -f [math]::Round($v.Length/1GB,2))

Write-Host "`n=== Lock state ==="
try {
    $s = [System.IO.File]::Open($v.FullName, 'Open', 'Read')
    $s.Close()
    Write-Host "UNLOCKED"
} catch {
    Write-Host "LOCKED: $($_.Exception.Message)"
}

Write-Host "`n=== handle owner search via handle.exe (if present) ==="
$handle = Get-Command handle.exe -ErrorAction SilentlyContinue
if ($handle) {
    & handle.exe -a -u "$($v.FullName)" 2>&1 | Select-Object -First 20
} else {
    Write-Host "handle.exe not available"
}
