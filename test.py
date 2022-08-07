#!/usr/bin/env python3
import os
import subprocess
from subprocess import run


BIN = './target/release/bf'

def recompile():
    run('cargo build --release'.split())

def test_hello():
    proc = run(f'{BIN} examples/hello.bf'.split(), capture_output=True)
    assert proc.stdout == b'Hello World!\n'
    print('PASS', 'hello')

def test_one_byte_adder():
    proc = run(f'{BIN} examples/one-byte-adder.bf'.split(), capture_output=True)
    assert proc.stdout == b'\x07'
    print('PASS', 'one byte adder')

def test_mandelbrot():
    proc = run(['bash', '-c', f'{BIN} examples/mandelbrot-titannic.bf | head -n 50 | sha256sum'], capture_output=True)
    assert b'2f9e8599109e0dec2dcd' in proc.stdout
    print('PASS', 'mandelbrot')

def test_combine_inc_dec():
    proc = run(f'{BIN} tests/combine-inc-dec.bf'.split(), capture_output=True)
    assert proc.stdout == b'\x03'
    print('PASS', 'combine inc dec')

def test_skip_loop():
    proc = run(f'{BIN} tests/skiploop.bf'.split(), capture_output=True)
    assert proc.stdout == b'OK\n'
    print('PASS', 'skip loop')



def main(do_slow):
    recompile()
    test_hello()
    test_one_byte_adder()
    test_combine_inc_dec()
    test_skip_loop()
    if do_slow:
        test_mandelbrot()
    return 0

if __name__ == '__main__':
    do_slow = not not os.getenv('SLOW', default=False)
    exit(main(do_slow))
