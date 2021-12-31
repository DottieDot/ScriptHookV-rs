# scripthookv-rs

Scripthookv-rs provides Rust bindings for ScriptHookV and GTA 5 natives.

**DISCLAIMER: This crate is still in early development and API is subject to change.**

## Crates

### libscripthookv-sys
This crate contains the raw bindings for ScriptHookV without any wrapping.

#### Installation
Add the following to your `cargo.toml`:

```toml
libscripthookv-sys = "0.1.0"
```

### scripthookv-rs
This crate contains a wrapped api for ScriptHookV

#### Installation
Add the following to your `cargo.toml`:

```toml
scripthookv-rs = "0.2.1"
```

### scripthookv-rs-gta
This crate contains a wrapped api for GTA 5 natives. It should be installed together with the `scripthookv-rs` crate.

#### Installation
This crate has not been published yet.

## Usage

### Entrypoint

For your entrypoint you can use the `shv_entrypoint` macro. This macro generates a DllMain for you.
```rs
extern "C" fn script_main() {
  loop {
    // On tick logic

    script_yield();
  }
}

#[shv_entrypoint]
fn entrypoint(module: ModuleHandle) -> ScriptHookV {
  ScriptHookVBuilder::new(module)
    .script(script_main)
    .build()
}
```

### Natives
Natives can be invoked directly with the `native_call` macro like this:
```rs
call_native!(Vehicle, 0xAF35D0D2583051B0, adder_hash, coords, heading, false, false, false);
//           ^^^^^^^  ^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//       Return type         Native hash                                        Parameters
```

Alternatively you can use [my nativedb](https://nativedb.dotindustries.dev/generate-code/rust) to generate wrapping similar to a `natives.h` file.

## Road map
- [x] Wrap all ScriptHookV functions\*.
- [ ] Dedicated readmes for all crates.
- [ ] More elegant wrapping for ScriptHookV.
- [ ] Provide elegant API for most natives.
- [ ] Provide a UI crate similar to [LemonUI](https://github.com/justalemon/LemonUI).
- [ ] Automated tests.

\* excluding deprecated functions.
## Contributing
Pull requests are always welcome. For major changes, please open an issue first to discuss what you would like to change.

## Acknowledgements

- [ScriptHookVDotNet](https://github.com/crosire/scripthookvdotnet/) for being a big inspiration for the wrapped native API.
- [Salted](https://github.com/VerySalted), [Daniel](https://github.com/DanielMaywood), and [ikt](https://github.com/E66666666) for general help.

## License
[MIT](https://choosealicense.com/licenses/mit/)
