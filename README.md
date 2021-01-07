# Brainfuck Interpreter, Matt Traudt (pastly)

Currently this code serves as a minimally-viable product version of a brainfuck
interpreter.

The only optimization I'm currently planning on:

- coalescing repeated instructions,
- transforming trivial/obvious instruction series (e.g. `[-]`) into a
  pseudo-instruction

Next steps (if I don't just forget about this and do other things):

- write a script to test my interpreter behaves the same as some other popular
  interpreter(s).

# Examples

Some or all of the examples are taken from
[this repo](https://github.com/fabianishere/brainfuck).

# Running

    $ cargo run --release -- examples/hello.bf
    $ cargo run --release -- examples/head.bf < src/bin/bf.rs

# License

This work is released to the public domain "under" the unlicense. See
`LICENSE`.
