// Created by Sean L. on Jun. 14.
// Last Updated by Sean L. on Jun. 14.
//
// stacky
// src/io.rs
//
// Makabaka1880, 2026. All rights reserved.

use crate::parser;

const RED: &str = "\x1b[0;31m";
const GREEN: &str = "\x1b[0;32m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

pub fn print_err(input: &str) {
    println!("{BOLD}{RED}[ERR!] {input}{RESET}");
}

pub fn print_succ(input: &str) {
    println!("{BOLD}{GREEN}[SUCC] {input}{RESET}");
}

pub fn print_tree(tree: &parser::Tree) {
    print_tree_helper(tree, 0);
}

fn print_tree_helper(tree: &parser::Tree, depth: usize) {
    for _ in 0..depth {
        print!("| ");
    }

    match tree {
        parser::Tree::Leaf(v) => {
            println!("⎣ {v}");
        }
        parser::Tree::Branch(v, children) => {
            println!("⎣ {v}");

            children
                .iter()
                .for_each(|child| print_tree_helper(child, depth + 1));
        }
    }
}