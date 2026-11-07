$cur = [Security.Principal.WindowsIdentity]::GetCurrent()
$prin = New-Object Security.Principal.WindowsPrincipal($cur)
Write-Host "User: $($cur.Name)"
Write-Host "Groups:"
$cur.Groups | ForEach-Object {
    $name = ([Security.Principal.SecurityIdentifier]$_.Value).Translate([Security.Principal.NTAccount])
    Write-Host "  $name"
}
Write-Host "`nIsInRole Administrator: $($prin.IsInRole([Security.Principal.WindowsBuiltinRole]::Administrator))"
Write-Host "IsInRole PowerUser: $($prin.IsInRole([Security.Principal.WindowsBuiltinRole]::PowerUser))"
