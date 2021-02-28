# hanzi_ime

Plain-text IME back-end to translate from ASCII pinyin to Unicode hanzi


## Project Status: Alpha

*This is project not ready for use yet.*


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


## Credits

This project builds on the work of others. See [CREDITS.md](CREDITS.md)


## License

Dual licensed under the terms of [Apache 2.0](LICENSE-APACHE) or
[MIT](LICENSE-MIT), at your option.
