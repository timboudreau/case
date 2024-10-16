Case-Convert
============

A dirt-simple command-line app for performing (sometimes exotic) case-conversions - just
a command-line interface to the [`case_convert`](https://crates.io/crates/case_convert)
Rust crate.  Operates on either trailing command-line arguments, or if none, stdin.

```sh
$ case-convert -c kebab ZenMultibandCompressorExtension
zen-multiband-compressor-extension

$ case-convert -c title ZenMultibandCompressorExtension
Zen Multiband Compressor Extension

$ case-convert -c snake ZenMultibandCompressorExtension
zen_multiband_compressor_extension

$ case-convert -c train ZenMultibandCompressorExtension
Zen-Multiband-Compressor-Extension
```

This kind of conversion is handy in shell scripts, code generation and generally things
that convert between programmatic names and display names.  There are, of course, incantations
with `sed` and similar that can do this sort of thing - this is just a straightforward,
non-abstruse way to do it.

The [help content](https://github.com/timboudreau/case/blob/main/src/help.txt) explains the command-line arguments in detail.


Install
-------

Simply clone, cd to the directory and run `cargo build --release` - then
put `target/release/case-convert` somewhere on your path.


### Input Handling

Trailing arguments which are not line-switches are treated as *separate* inputs
to process, and emitted space-separated (any line switches after the first 
unrecognized argument are treated as input) rather than treated as a single
string.  Simply quote the string if you want to pass it as a single argument.

The upshot of this is 

```sh
case-convert -c snake wurgle_burgle-boo -c alternating FizzBin
```

results in the output *wurgle_burgle_boo c alternating fizz_bin*, but

```sh
case-convert -c snake "wurgle_burgle-boo -c Alternating FizzBin"
```

results in the output *wurgle_burgle_boo_c_alternating_fizz_bin*

Stdin-based input is handled a line at a time.


License
-------

Licensed under the [MIT license](https://opensource.org/license/mit).
