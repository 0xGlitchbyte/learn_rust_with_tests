# Install Rust

The official instructions to install Rust can be found [here](https://www.rust-lang.org/tools/install).

## Installation

Rust is installed through the `rustup` command line tool, which allows us to manage Rust versions and associated tools. It installs the Rust toolchain, `rustc`compiler, `cargo` tool, and much more.

### Install on Linux

If you're using linux, you can open a terminal and enter the following command:

`$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

This command downloads the `rustup` install script and runs it, which will then install Rust. You might be prompted to enter your password. If the install was successful, you should see the following message:

`Rust is installed now. Great!`

To verify the script ran successfully, we can type the following command to get the `rustup` version: 

`$ rustup --version`

### Install on Mac

If you're using a mac, you first need to install a C compiler. We'll do that using xcode:

`$ xcode-select --install`

Similar to linux, we will open a terminal and enter the command for `rustup`:

`$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

We can verify with:

`rustup --version`

### Install on Windows

Installing on windows will be slightly different. You'll need to download the [rustup.exe](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe) and run it.

You may need to install the Visual C++ Build Tools 2019 or equivalent (Visual Studio 2019, etc.) if theyre not already installed.
