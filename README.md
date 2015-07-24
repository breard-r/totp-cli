# totp-cli

totp-cli is a minimalist TOTP client without key persistence. It reads a base-32 encoded secret key from the standard input and then prints the corresponding TOTP code.


## Compiling from Source

To build totp-cli, you will need the [rust compiler](https://github.com/rust-lang/rust) and [cargp](https://github.com/rust-lang/cargo).

    git clone https://github.com/breard-r/totp-cli
    cd totp-cli
    cargo build --release

The resulting binary will be located in `./target/release/totp-cli`.


## Usage example

You can store all your secret keys using [pass](http://www.passwordstore.org/) and pipe it to totp.py:

    pass show example.com/totp | totp-cli
