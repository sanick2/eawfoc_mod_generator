# DEV WORKFLOW

# use Bash elsewhere
[linux]
set shell := ["bash", "-euxo", "pipefail", "-c"]

# use Powershell 5.1 in Windows 11
[windows]
set shell := ["powershell.exe", "-NoLogo", "-Command"]

default:
	just --list --unsorted
	
build-debug:
    cargo build 

build-release:
    cargo build --release

clean:
    cargo clean

run:
    cargo run