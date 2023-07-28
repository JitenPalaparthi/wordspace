### enable logger

    ```
    export RUST_LOG=wordspace=debug
    ```

### setup port from environment variables

```
exprt WORDSPACE_PORT=8085
```

### make sure only owners of the directory can perfrom operations

```
chmod 700 datastore
```

### make wordspace program the owner of datastore


### cross compile

- To cross compile , you need to know target triple (which is from llvm).

- List of few cross compile targets 

| Target-triple-name       |                     | Description                             |
|--------------------------|---------------------|-----------------------------------------|
| x86_64-unknown-linux-gnu |                     | 64-bit Linux (kernel 3.2+, glibc 2.17+) |
| x86_64-pc-windows-msvc   |                     | 64-bit MSVC (Windows 7+)                |
| x86_64-apple-darwin      |                     | 64-bit macOS (10.7+, Lion+)             |
| aarch64-unknown-linux-gnu|                     | ARM64 Linux (kernel 4.1, glibc 2.17+)   |
| aarch64-apple-darwin     |                     | ARM64 macOS (11.0+, Big Sur+)           |
| aarch64-apple-ios        |                     | ARM64 iOS                               |
| aarch64-apple-ios-sim    |                     | Apple iOS Simulator on ARM64            |
| armv7-linux-androideabi  |                     | ARMv7a Android                          |

- Install cross crate. This executable is stored i $HOME/.cargo/bin.

```
cargo install cross
```

- To cross compile and run

```
cargo run --target x86_64-unknown-linux-gnu
```

- To build

```
cargo build
```
- To build in release mode

```
cargo build --release
```