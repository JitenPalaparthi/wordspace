1. If the function is directly in lib.rs then can call in main as

    - wordspace is name of the project in cargo.toml
    - hello is a function in lib.rs

```
use wordspace::hello
```
2. If a new file is created example routes.rs in src/ then
    - each file in in src is a module.All public types, functions , methods etc.. can be called.

```
use routes; // in main and call that function directly with the module name.

```
3. If a directory is created for a package, then make sure to use mod.rs file inside it.
