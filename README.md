# Stacky
A simple stack-based language for educational purposes.

> Stacky is isomorphic to RPN when not using pop — a nice example of using programs to describe expressions. Formal syntax and semantics are in [`lang/theory.typ`](lang/theory.typ).

## Concrete Syntax

The interpreter uses an S-expression surface syntax:

```
stmt    ::= (push <value>)
        |   (pop)
        |   (squish <value> <int>)
prog    ::= (begin <prog1> <prog2> ...)
        |   stmt
```

`<value>` is any valid identifier. Nested `begin` blocks are flattened into a linear program at parse time.

## Quick Start

```bash
# Native CLI
cargo run -- '(begin (push hello) (push world) (squish pair 2))'

# Build WASM
wasm-pack build --target web

# Playground (after building WASM)
python3 -m http.server 8080
open http://localhost:8080/www
```

## Project Structure

```
src/
  lib.rs         — WASM bindings (Program class, get_top)
  main.rs        — Native CLI entry point
  interpreter.rs — Config / step / run_all
  parser.rs      — S-expression parser (Stmt, Tree)
  io_utils.rs    — Pretty-printing
www/
  index.html     — Browser playground
lang/
  theory.typ     — Formal syntax & small-step semantics
```
