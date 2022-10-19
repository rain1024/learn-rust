# How to install Rust on MacOS

```
$ brew install rust
```

Rust Hello world 

`main.rs` 

```rs
fn main() {
    println!("Hello World!");
}
```

Build and run code 

```
$ rustc main.rs
$ ./hello
Hello World!
```

## Rust with Visual Studio Code (2022)

1. Get Visual Studio Code 1.72.1+
2. Change Indent to 4

Edit `settings.json` file (`Preferences: Open User Settings (JSON)`)

```
"[rust]": {
      "editor.defaultFormatter": "rust-lang.rust-analyzer",
      "editor.tabSize": 4
    }
```

Now you can format rust file with `Control + Shift + I`

3. Extensions

* [`rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) - Rust language support for Visual Studio Code (better than [`Rust` extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust))
* [`crates`](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates) - Helps Rust developers managing dependencies with Cargo.toml
* [`Tabnine AI`](https://marketplace.visualstudio.com/items?itemName=TabNine.tabnine-vscode) - AI Code completion plugin
* [`Error Lens`](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens) - Improve highlighting of errors, warnings and other language diagnostics.