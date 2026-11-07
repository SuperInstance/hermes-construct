# COM7 probe — talk to the board's serial console from WSL via Windows PowerShell
$port = New-Object System.IO.Ports.SerialPort('COM7', 115200, 'None', 8, 'One')
$port.ReadTimeout = 1500
$port.NewLine = "`n"
try {
  $port.Open()
  Start-Sleep -Milliseconds 500
  $port.Write("`r`n")
  Start-Sleep -Milliseconds 800
  $out = ""
  try { while ($true) { $out += $port.ReadLine() + "`n" } } catch {}
  $port.Write("`r`n")
  Start-Sleep -Milliseconds 800
  try { while ($true) { $out += $port.ReadLine() + "`n" } } catch {}
  Write-Output "===SERIAL-OUTPUT==="
  Write-Output $out
  Write-Output "===END==="
} catch {
  Write-Output "ERROR: $($_.Exception.Message)"
} finally { if ($port.IsOpen) { $port.Close() } }
