$currentPath = [Environment]::GetEnvironmentVariable('Path', 'Machine')
$cleaned = ($currentPath -split ';' | Where-Object { $_ -ne 'C:\Program Files\GitHub CLI\' -and $_ -ne 'C:\Program Files\GitHub CLI' }) -join ';'
$finalPath = $cleaned + ';C:\Program Files\GitHub CLI'
[Environment]::SetEnvironmentVariable('Path', $finalPath, 'Machine')
Write-Host "PATH has been updated. GitHub CLI path is now: C:\Program Files\GitHub CLI"
