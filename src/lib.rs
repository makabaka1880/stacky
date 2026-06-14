// Created by Sean L. on Jun. 14.
// Last Updated by Sean L. on Jun. 14.
//
// stacky
// src/lib.rs
//
// Makabaka1880, 2026. All rights reserved.

mod interpreter;
mod io_utils;
mod parser;

use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;

/// A Stacky program that can be executed incrementally.
///
/// JS usage:
/// ```js
/// const prog = new Program("(begin (push a) (push b) (squish pair 2))");
/// while (!prog.done()) {
///     prog.step();
///     console.log(prog.stack_len(), prog.peek(0));
/// }
/// const tree = prog.run_all(); // one-shot alternative
/// ```
#[wasm_bindgen]
pub struct Program {
    inner: interpreter::Config,
}

#[wasm_bindgen]
impl Program {
    /// Parse a Stacky source program. Throws a JS error on parse failure.
    #[wasm_bindgen(constructor)]
    pub fn new(source: &str) -> Result<Program, JsValue> {
        interpreter::Config::parse_program(source)
            .map(|inner| Program { inner })
            .map_err(|e| JsValue::from_str(&e))
    }

    /// Execute a single statement. Returns `true` when the program is done,
    /// `false` when more steps remain. Throws on runtime errors.
    pub fn step(&mut self) -> Result<bool, JsValue> {
        self.inner.step().map_err(|e| JsValue::from_str(&e))
    }

    /// Run the program to completion and return the top-of-stack tree as a JS object.
    /// Throws on runtime errors or if the stack is empty when the program finishes.
    pub fn run_all(&mut self) -> Result<JsValue, JsValue> {
        self.inner.run_all().map_err(|e| JsValue::from_str(&e))?;
        match self.inner.peek_tree(0) {
            Some(tree) => serde_wasm_bindgen::to_value(tree).map_err(JsValue::from),
            None => Err(JsValue::from_str("program finished with empty stack")),
        }
    }

    /// Has the program exhausted all statements?
    #[wasm_bindgen(getter)]
    pub fn done(&self) -> bool {
        self.inner.is_done()
    }

    /// Number of items currently on the stack.
    pub fn stack_len(&self) -> usize {
        self.inner.stack_len()
    }

    /// Peek at the stack from the top: depth 0 = top, depth 1 = one below, etc.
    /// Returns the tree as a JS object, or `undefined` if depth is out of bounds.
    pub fn peek(&self, depth: usize) -> JsValue {
        match self.inner.peek_tree(depth) {
            Some(tree) => serde_wasm_bindgen::to_value(tree).unwrap_or(JsValue::UNDEFINED),
            None => JsValue::UNDEFINED,
        }
    }
}

/// Convenience: parse source, run to completion, return the result tree as a debug string.
/// Prefer the `Program` class for interactive use.
#[wasm_bindgen]
pub fn get_top(source: &str) -> String {
    let mut program = match interpreter::Config::parse_program(source) {
        Ok(prog) => prog,
        Err(e) => return format!("Parse error: {e}"),
    };
    loop {
        match program.step() {
            Ok(true) => break,
            Ok(false) => continue,
            Err(e) => return format!("Runtime error: {e}"),
        }
    }
    format!("{:?}", program.get_front())
}
