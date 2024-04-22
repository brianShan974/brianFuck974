# brianFuck974
brianFuck974, named after me. My implementation of brainfuck in rust. In this implementation, 32768 32-bit cells are used.

In order to run this interpreter, use `cargo run`. You can also `cargo run -- --help` for some hints on how to use this interpreter.

It currently supports 2 modes, file mode and command line mode. Suppose that your program is in the file `example.b`, you can run
```
cargo run -- file example.b
```
to run the program.

You can also directly run a line of the program in the command line. For example, you can do this:
```
cargo run -- cl >>>>><<<<<+++++-----
```

By the way, rust is the best language ever.
