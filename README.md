# Platform-LP-RS
A platform enum written in *rust* to be used with *lovepack* tools.

Contains functions to determine the current running platform as well as parsing and comparisons.

## Usage

Include the library in your `cargo.toml`.

```toml
[dependencies]
platform-lp = "0.2"
```

Then use it in your library / application.

```rust
let user_plat = platform_lp::Platform::get_user_platform();

// you can pass an entire executable name to check what platform it is, 
// assuming it goes by some kind of standard naming convention.
// like perhaps one of mine: lpsettings-0.1.7-win-x86_64.zip
let package_platform = platform_lp::Platform::new(executable_release_name);

// then you can check if its the same platform
if user_plat == package_platform {
    // then extract it, run it, etc ...
}

// or you can check compatability (i.e. for 32 bit on 64 bit systems)
if package_platform.is_compatible(user_plat) {
    // then do something fun...
}

```
## Changes

### 0.2.1

- Platform now implements `std::fmt::Display`