# Rust and Wasm Code Size Benchmarks

A collection of (mostly micro) benchmark Rust programs to keep an eye on
WebAssembly code size.

Runs `wasm-gc` (to remove DWARF debugging custom sections) but does not run
`wasm-opt` or `wasm-snip` or any of the other common tools on the built `.wasm`
files.

Uses `opt-level = "s"` and `debug = false`.

## Run the Benchmarks

```
./sizes.sh
```

This will report the byte sizes of the emitted `.wasm` binaries to `stdout`.

## Synthetic Benchmarks

Very small, targeted, synthetic benchmarks.

* `allocator-system`: A `.wasm` binary that exports the system allocator's
  (which is dlmalloc for `wasm32-unknown-unknown` at the time of writing)
  functionality to the host.

* `allocator-wee-alloc`: The same as `allocator-system` but with `wee_alloc`.

* `panic-empty`: Exports a single function that invokes the `panic!()` macro
  without any arguments.

* `panic-in-bounds-check`: Exports a single function that constructs a slice and
  indexes into it in a way such that the bounds check can't be elided by the
  compiler, and so it might panic.

* `panic-in-unwrap`: Exports a single function that unwraps a result,
  potentially panicking.

* `panic-with-dynamic-message`: Exports a single function that invokes the
  `panic!()` macro with a dynamic string message.

* `panic-with-static-message`: Exports a single function that invokes the
  `panic!()` macro with a `&static str` message.

* `regular-expressions`: Exports a function that takes a regular expression
  string and an input string, uses the `regex` crate to compile the regular
  expression string, and then returns whether the input string is a match or
  not.

## Real World Libraries

* `source-map-mappings`: The WebAssembly core of the `mozilla/source-map`
  JavaScript library.
