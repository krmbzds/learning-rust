# Learning Rust

I'm going to go over [Rust by Example][rust_by_example] and commit relevant stuff here.

So I can check what I've done one day if I get \**ehm*\* Rusty.

## Getting our shit together

You need to install [Rust][rust_download_page] first.

I'm going to use [Atom][atom_editor] editor, for no particular reason.

As usual, I'm going to start by setting up an unnecessarily extravagant development environment. If you're on OSX or Linux you should be able to apply these steps without a problem. If you're on Windows: `¯\_(ツ)_/¯`.

Let's just install everything that doesn't have a dependency.

```sh
apm install language-rust         # Rust language support
apm install linter                # You probably have this
apm install linter-rust           # Rust linter
apm install rust-api-docs-helper  # Browse documentation with a click
apm install atom-browser-webview  # To browse documentation inside atom
```

Now we'll configure autocompletion using the `racer` package. This part is a little tricky. We first need to install the Cargo package, the Atom plugin, then we need to download and extract the Rust source code in a way racer can read it.

```sh
# Some variables
DIR="/usr/local/src/rust/src"
URL="https://static.rust-lang.org/dist/rustc-1.5.0-src.tar.gz"
FILE="rustc-1.5.0-src.tar.gz"

# For code completion
cargo install racer
apm install racer

# Download & extract Rust source
mkdir -p $DIR                   # Create some directory
wget -P $DIR $URL               # Download Rust source code
tar xvzf "$DIR/$FILE" -C $DIR   # Extract source code
rm "$DIR/$FILE"                 # Remove the archive

# Throw in some environment variables
echo 'export PATH="$PATH:$HOME/.cargo/bin" # To run Cargo binaries' >> ~/.profile
echo 'export RUST_SRC_PATH=/usr/local/src/rust/src # Declare Rust source path' >> ~/.profile
```

If you also want to auto-compile your code inside Atom, install:

```sh
apm install build
apm install build-cargo
```

If all went well, you should now have a *motherfuckingly badass* setup. Since you're doing everything inside the editor, you'll only have to leave it when you need to pee (unless you're coding in the toilet).


# License

[Mozilla Public License Version 2.0][license]

For the kicks!


[atom_editor]: https://atom.io/
[rust_by_example]: http://rustbyexample.com/index.html
[rust_download_page]: https://www.rust-lang.org/downloads.html
[license]: https://raw.githubusercontent.com/krmbzds/learning-rust/master/LICENSE
