# DEV WORKFLOW

# use Powershell 5.1 in Windows 11
set shell := ["powershell.exe", "-c"]

build-debug:
    cargo build 

build-release:
    cargo build --release

clean:
    cargo clean

run:
    cargo run