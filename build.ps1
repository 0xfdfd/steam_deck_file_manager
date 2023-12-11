# Initialize flags
$run_prog = $false
$run_opt = "--"
$cargo_build_flags = ""
$wasm_pack_flags = "--dev"
$build_backend = $true
$build_frontend = $true
$build_failure = $false

# Process command line arguments
foreach ($arg in $args) {
    switch ($arg) {
        "--release" {
            $cargo_build_flags = "--release"
            $wasm_pack_flags = "--release"
        }
        "--no-gui" {
            $run_opt += " --no-gui"
        }
        "--run" {
            $run_prog = $true
        }
        "--frontend" {
            $build_backend = $false
            $build_frontend = $true
        }
        { @("-h", "--help") -contains $_ } {
            Write-Host "Usage:"
            Write-Host "`t--release      : Build in release mode."
            Write-Host "`t--no-gui       : Disable GUI."
            Write-Host "`t--run          : Set run flag."
            Write-Host "`t--frontend     : Set to build frontend and not backend."
            Write-Host "`t-h, --help     : Display this help message."
            exit
        }
    }
}

# Function to check if a command exists
function CommandExists($cmd) {
    $exists = $null -ne (Get-Command $cmd -ErrorAction SilentlyContinue)
    return $exists
}

# Install dependency
if (-not (CommandExists "wasm-pack")) {
    cargo install --locked wasm-pack
}
if (-not (CommandExists "wasm-bindgen")) {
    cargo install --locked wasm-bindgen-cli
}

# Build frontend if flag is true
if ($build_frontend) {
    Push-Location

    try {
        Set-Location frontend
        cargo fmt
        wasm-pack build $wasm_pack_flags --no-typescript --no-pack --target web
    }
    catch {
        $build_failure = $true
    }
    finally {
        Pop-Location
    }

    if ($build_failure) {
        exit
    }
}

# Build backend if flag is true
if ($build_backend) {
    cargo fmt
    cargo build $cargo_build_flags
}

if ($run_prog) {
    cargo run $cargo_build_flags $run_opt
}
