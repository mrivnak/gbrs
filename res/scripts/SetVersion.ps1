$version = $args[0].replace("v", "")

$file = ".\gbrs\src\Cargo.toml"

((Get-Content -path $file -Raw) -replace '0.0.0',$version) | Set-Content -Path $file