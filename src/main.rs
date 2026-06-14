// Created by Sean L. on Jun. 14.
// Last Updated by Sean L. on Jun. 14.
//
// stacky
// src/main.rs
//
// Makabaka1880, 2026. All rights reserved.

use std::io::Read;

mod interpreter;
mod io_utils;
mod parser;

fn main() {
    let mut input = String::new();
    let mut handle = std::io::stdin().lock();

    if let Err(e) = handle.read_to_string(&mut input) {
        io_utils::print_err(e.to_string().as_str());
        return;
    }

    let mut program = match interpreter::Config::parse_program(&input) {
        Ok(prog) => prog,
        Err(e) => {
            io_utils::print_err(&e);
            return;
        }
    };

    // Execute step by step, printing the stack top after each step
    while !program.is_done() {
        io_utils::print_succ("Exec...");
        if let Err(e) = program.step() {
            io_utils::print_err(&e);
            return;
        }
        let front = program.get_front();
        io_utils::print_tree(front);
    }
}
