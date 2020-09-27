# Meiling On Windows

## GNU
### MSYS2 (MinGW)
1. Install MSYS2 via scoop or msi installer
2. Update packages via `pacman -Syu`
3. Restart MSYS2 shell
4. Run 2 again.
5. run `pacman -S --needed base-devel mingw-w64-x86_64-toolchain mingw-w64-x86_64-cmake`
6. Register MinGW toolchain directory to `env:PATH`

### Rustup
1. download and run [rustup](https://rustup.rs)
2. Select 2 (Customize installation)
    * Set host triple to `x86_64-pc-windows-gnu`
3. Install rust toolchain

### MySQL Libraries
1. Download and unzip MySQL Server 5
2. Unzip it and copy /lib location (it's like `_location_/lib`)
3. Register it to `env:PATH` and `env:MYSQLCLIENT_LIB_DIR`
4. Try build meiling now
