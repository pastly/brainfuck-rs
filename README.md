# Brainfuck Interpreter, Matt Traudt (pastly)

Currently this code serves as a minimally-viable product version of a brainfuck
interpreter.

The only optimization I'm currently planning on:

- coalescing repeated instructions,
- transforming trivial/obvious instruction series (e.g. `[-]`) into a
  pseudo-instruction

# Running

    $ cargo run --release -- examples/hello.bf
    $ cargo run --release -- examples/head.bf < src/main.rs

# License

This work is released to the public domain "under" the unlicense. See
`LICENSE`.
