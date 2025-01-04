# brianFuck974

brianFuck974,
named after me.
My implementation of brainfuck in rust.
In this implementation,
initially 32 128-bit cells are used.
The cells are actually stored in a `Vec` of `i128`s,
so it could be extended whenever needed.

In order to run this interpreter,
you can use `cargo run`.
You can also `cargo run -- --help` for some hints on how to use this interpreter.
Suppose that your program is in the file `example.bf`,
you can run

```
cargo run -- example.bf
```

to run the program.

To install this interpreter on the system,
run

```
cargo install --path .
```

in the project directory.
After installation,
you can run `brianfuck974` to call this interpreter.
If you want to change the name of the command,
please refer to `Cargo.toml` and change the `name` under `[[bin]]` before installation.

By the way,
rust is the best language ever.
