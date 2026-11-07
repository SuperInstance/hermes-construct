$base = "C:\reseachlocal"
$csv  = "C:\Users\casey\upstream_check.csv"

if (Test-Path $csv) { Remove-Item $csv }

$folderList = Get-ChildItem $base -Directory -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Name

$skip = @('PersonalLog','activelog-backend','activelog-claude','ActiveLog','ActiveLog-MVP','ActiveLog-TechnicalRepo',
          'activelog2','activelogearlier','BusinessLog','businesslog-app','MakerLog','PlayerLoG.AI','voice-note-logger',
          'consolidated-documents','PROJECT_AUDITS','superinstance-audit','superinstance-docs',
          'New folder (2)','New folder (4)','project archive','smartCDRT backup','wslback1.11',
          'montized super','What is this file doing here','podcast1','podcastz',
          '.nuget','.templateengine','.theia-ide','Mycelium_Final_Research_Package')

$candidates = $folderList | Where-Object { $_ -notin $skip }

"Name,Status,Remote,LocalDate,RemoteDate,Behind,Ahead,Details" | Out-File $csv -Append

function Append-Row($name,$status,$remote,$localD,$remoteD,$behind,$ahead,$details) {
    $remote  = ($remote  -replace ',',';')
    $details = ($details -replace ',',';')
    "$name,$status,$remote,$localD,$remoteD,$behind,$ahead,$details" | Out-File $csv -Append
    Write-Host "[$status] $name"
}

function Run-Git($dir, [string[]]$gitArgs, [int]$timeoutSec = 15) {
    $argStr = ($gitArgs | ForEach-Object { if ($_ -match '\s') { '"' + ($_ -replace '"','\"') + '"' } else { $_ } }) -join ' '
    $psi = New-Object System.Diagnostics.ProcessStartInfo
    $psi.FileName = 'git.exe'
    $psi.WorkingDirectory = $dir
    $psi.RedirectStandardOutput = $true
    $psi.RedirectStandardError  = $true
    $psi.UseShellExecute = $false
    $psi.CreateNoWindow = $true
    $psi.Arguments = $argStr
    $p = [System.Diagnostics.Process]::Start($psi)
    if (-not $p.WaitForExit($timeoutSec * 1000)) {
        try { $p.Kill($true) } catch {}
        return @{ ExitCode = -1; StdOut = $p.StandardOutput.ReadToEnd(); StdErr = ($p.StandardError.ReadToEnd() + ' TIMEOUT') }
    }
    return @{ ExitCode = $p.ExitCode; StdOut = $p.StandardOutput.ReadToEnd(); StdErr = $p.StandardError.ReadToEnd() }
}

# Make sure SSH agent has a key (best-effort)
$agentKeys = Get-Process ssh-agent -ErrorAction SilentlyContinue
# leave this to the user's existing setup; we won't try to add keys.

$counter = 0
foreach ($name in $candidates) {
    $counter++
    $dir = Join-Path $base $name
    if (-not (Test-Path $dir)) { Append-Row $name 'MISSING' '' '' '' '' '' 'folder gone'; continue }
    if (-not (Test-Path (Join-Path $dir '.git'))) { Append-Row $name 'NOT_GIT' '' '' '' '' '' 'no .git directory'; continue }

    # Get remote URL
    $r = Run-Git $dir @('remote','get-url','origin') 5
    if ($r.ExitCode -ne 0 -or [string]::IsNullOrWhiteSpace($r.StdOut)) {
        Append-Row $name 'NO_REMOTE' '' '' '' '' '' 'no remote.origin.url'
        continue
    }
    $remote = ($r.StdOut.Trim() -split "`r?`n")[0]

    # If SSH, switch to HTTPS automatically (read access via HTTPS is fine for public repos)
    if ($remote -match '^git@github\.com:(.+?)(?:\.git)?$') {
        $pathOnly = $matches[1]
        if (-not $pathOnly.EndsWith('.git')) { $pathOnly += '.git' }
        $httpsUrl = "https://github.com/$pathOnly"
        $sw = Run-Git $dir @('remote','set-url','origin', $httpsUrl) 5
        if ($sw.ExitCode -eq 0) { $remote = $httpsUrl }
    }

    # Local HEAD
    $lr = Run-Git $dir @('log','-1','--format=%ci|%H') 5
    $localDate = ''; $localHash = ''
    if ($lr.ExitCode -eq 0 -and $lr.StdOut) {
        $parts = ($lr.StdOut.Trim() -split "`r?`n")[0] -split '\|',2
        $localDate = $parts[0]; $localHash = $parts[1]
    }

    # Fetch
    $fr = Run-Git $dir @('fetch','--quiet','--no-tags','--depth=1') 25
    if ($fr.ExitCode -ne 0) {
        $err = ($fr.StdErr + ' ' + $fr.StdOut).Trim()
        if ($err.Length -gt 150) { $err = $err.Substring(0,150) }
        Append-Row $name 'FETCH_FAIL' $remote $localDate '' '' '' $err
        continue
    }

    # Default branch
    $db = Run-Git $dir @('symbolic-ref','--short','refs/remotes/origin/HEAD') 5
    $defBranch = ''
    if ($db.ExitCode -eq 0 -and $db.StdOut) { $defBranch = $db.StdOut.Trim() }
    if (-not $defBranch) {
        $t1 = Run-Git $dir @('rev-parse','--verify','--quiet','origin/main') 3
        if ($t1.ExitCode -eq 0 -and $t1.StdOut) { $defBranch = 'origin/main' }
    }
    if (-not $defBranch) {
        $t2 = Run-Git $dir @('rev-parse','--verify','--quiet','origin/master') 3
        if ($t2.ExitCode -eq 0 -and $t2.StdOut) { $defBranch = 'origin/master' }
    }
    if (-not $defBranch) {
        Append-Row $name 'NO_DEFAULT_BRANCH' $remote $localDate '' '' '' 'no origin/HEAD or origin/main or origin/master'
        continue
    }

    $rr = Run-Git $dir @('log','-1','--format=%ci|%H', $defBranch) 5
    $remoteDate = ''; $remoteHash = ''
    if ($rr.ExitCode -eq 0 -and $rr.StdOut) {
        $parts = ($rr.StdOut.Trim() -split "`r?`n")[0] -split '\|',2
        $remoteDate = $parts[0]; $remoteHash = $parts[1]
    }

    $behindR = Run-Git $dir @('rev-list','--count','HEAD..' + $defBranch) 10
    $aheadR  = Run-Git $dir @('rev-list','--count', $defBranch + '..HEAD') 10
    $behind = if ($behindR.ExitCode -eq 0) { ($behindR.StdOut.Trim() -split "`r?`n")[0] } else { '' }
    $ahead  = if ($aheadR.ExitCode  -eq 0) { ($aheadR.StdOut.Trim()  -split "`r?`n")[0] } else { '' }

    $status = 'OK'; $details = ''
    if (-not $localHash -or -not $remoteHash) {
        $status = 'UNKNOWN'; $details = 'missing hash info'
    } elseif ($localHash -eq $remoteHash) {
        $status = 'UP_TO_DATE'
    } elseif ([int]$behind -eq 0 -and [int]$ahead -gt 0) {
        $status = 'AHEAD'; $details = "ahead $ahead"
    } elseif ([int]$behind -gt 0 -and [int]$ahead -eq 0) {
        $status = 'BEHIND'; $details = "behind $behind"
    } else {
        $status = 'DIVERGED'; $details = "ahead $ahead / behind $behind"
    }

    Append-Row $name $status $remote $localDate $remoteDate $behind $ahead $details
}

Write-Host ""
Write-Host "=== DONE: processed $counter folders ==="
