$psi = New-Object System.Diagnostics.ProcessStartInfo
$psi.FileName = "git"
$psi.WorkingDirectory = "C:\reseachlocal\agent-grid"
$psi.RedirectStandardOutput = $true
$psi.RedirectStandardError  = $true
$psi.UseShellExecute = $false
$psi.CreateNoWindow = $true
[void]$psi.ArgumentList.Add("remote")
[void]$psi.ArgumentList.Add("-v")
$p = [System.Diagnostics.Process]::Start($psi)
$p.WaitForExit(5000) | Out-Null
Write-Host "ExitCode: $($p.ExitCode)"
Write-Host "StdOut: [$($p.StandardOutput.ReadToEnd())]"
Write-Host "StdErr: [$($p.StandardError.ReadToEnd())]"
