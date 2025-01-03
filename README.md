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

By the way,
rust is the best language ever.
