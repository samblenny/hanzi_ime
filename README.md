# hanzi_ime

Plain-text IME back-end to translate from ASCII pinyin to Unicode hanzi


## Project Status: Beta-ish

**This works, but it's being refactored. The API is not stable yet.**


## Language Support

Simplified Chinese: 2500 word HSK5 vocabulary


## Usage

To use hanzi_ime:

1. Prepare a front-end UI that can:
   * Receive keystrokes (readline style line buffered input won't work)
   * Buffer the keystrokes into a string slice for input to hanzi_ime
   * Display an output string from hanzi_ime after each keystroke
   * Allow the user to do something with the output string once it is ready:
     for example, pressing return could copy the output string to an editor
     buffer and clear the input string

2. After each input keystroke to the front-end UI, call hanzi_ime with the
   input string and display its response in the UI's output area


## WebAssembly Demo

Hosted: https://samblenny.github.io/hanzi_ime/wasm_demo/

Local (requires ruby and make):

```
cd wasm_demo
# Build wasm32-unknown-unknown binary and copy to ./
make install
# Start a dev webserver to serve ./ on http://localhost:8000
ruby webserver.rb
```


## Developer Tools Setup

| Tool | Purpose |
|--|--|
| rustup | Get rustc, cargo, and wasm32-unknown-unknown |
| ruby v2.6+ | Local web server for WebAssembly Demo + Rust code generation from vocab lists |
| GNU make | Augment cargo for building the wasm demo |

1. Install rustc with rustup. See <https://www.rust-lang.org/tools/install>
2. Configure PATH environment variable: add `export PATH="$PATH:$HOME/.cargo/bin"`
   to .bash_profile or whatever
3. Add WebAssembly compile target: `rustup target add wasm32-unknown-unknown`
4. Make sure you have a ruby interpreter v2.6 or later: `ruby --version`
   - For macOS, the default ruby should work fine (may need to install xcode)
   - Debian may need `sudo apt install ruby`


## Tests

### Library

From repository root directory:
```
cargo test
```


### Wasm Demo

From repository root directory:
```
cd wasm_demo
# Note: this uses *make* to call cargo test with extra args
make test
```


## Build and Run WebAssembly Demo

1. From repository root directory:
   ```
   cd wasm_demo
   make test
   make install
   make webserver
   ```
   Note: `make webserver` is a shortcut for `ruby webserver.rb`
2. Load http://localhost:8000 in browser
3. Stop `webserver.rb` with control-c when done


### Customize Vocab List

1. Read `vocab/autogen-hsk.rb`. There is an array near the top to set which .tsv
   files contain vocab words. Comments describe how the .tsv fields are used.
2. On macOS, BBEdit works well for editing .tsv files. It helps to set 36 pt font
   and 12 character tab width.
3. To re-generate the vocab data static arrays in `src/autogen_hsk.rs`:
   ```
   cd vocab/
   ruby autogen-hsk.rb
   ```
   By default, the script just checks the contents of the .tsv files for duplicates
   and other problems. To update `autogen_hsk.rs`, you must answer `y` at prompt.


## Credits

This project builds on the work of others. See [CREDITS.md](CREDITS.md)


## License

Dual licensed under the terms of [Apache 2.0](LICENSE-APACHE) or
[MIT](LICENSE-MIT), at your option.
