# Finds Inno Setup on the runner, installing the pinned version through Chocolatey only if it is missing.
$candidates = @("${env:ProgramFiles(x86)}\Inno Setup 6\ISCC.exe", "$env:ProgramFiles\Inno Setup 6\ISCC.exe", "$env:LOCALAPPDATA\Programs\Inno Setup 6\ISCC.exe")
$iscc = $candidates | Where-Object { Test-Path $_ } | Select-Object -First 1
if (-not $iscc) {
  choco install innosetup --version 6.7.3 -y --no-progress
  if ($LASTEXITCODE -ne 0) { throw "Inno Setup install failed" }
  $iscc = $candidates | Where-Object { Test-Path $_ } | Select-Object -First 1
}
if (-not $iscc) { throw "ISCC.exe not found" }
"ISCC=$iscc" >> $env:GITHUB_ENV
$env:ISCC = $iscc
