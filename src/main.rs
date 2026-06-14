// Created by Sean L. on Jun. 14.
// Last Updated by Sean L. on Jun. 14.
//
// stacky
// src/main.rs
//
// Makabaka1880, 2026. All rights reserved.

use std::io::Read;

use crate::{interpreter::Config, io_utils::print_tree, parser::parse};

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

    let sexp_repr = match sexp::parse(&input) {
        Ok(sexp) => sexp,
        Err(e) => {
            io_utils::print_err(&e.to_string().as_str());
            return;
        }
    };

    let ast = match parse(&sexp_repr) {
        Ok(ast) => ast,
        Err(e) => {
            io_utils::print_err(&e.to_string().as_str());
            return;
        }
    };

    let mut program = interpreter::Config::new(ast);
    while !match program.step() {
        Ok(b) => b,
        Err(e) => {
            io_utils::print_err(&e.to_string().as_str());
            return;
        }
    } {
        io_utils::print_succ("Exec...");
        let back = program.get_front();
        io_utils::print_tree(back);
    }
}