// Created by Sean L. on Jun. 14.
// Last Updated by Sean L. on Jun. 14.
//
// stacky
// src/interpreter.rs
//
// Makabaka1880, 2026. All rights reserved.

use crate::parser::{self, Stmt, Tree};

pub struct Config {
    program: Vec<Stmt>,
    pub store: Vec<Tree>,
}

impl Config {
    pub fn new(prog: Vec<Stmt>) -> Config {
        Config {
            program: prog,
            store: Vec::new(),
        }
    }
    pub fn step(&mut self) -> Result<bool, String> {
        if self.program.is_empty() {
            return Ok(true);
        }
        let stmt = self.program.remove(0);
        match stmt {
            Stmt::Push(v) => self.store.push(Tree::Leaf(v)),
            Stmt::Pop => match self.store.pop() {
                Some(_) => (),
                None => return Err("pop on empty stack".into()),
            },
            Stmt::Squish(v, n) => {
                let mut children = Vec::new();
                for _ in 0..n {
                    children.push(match self.store.pop() {
                        Some(child) => child,
                        None => return Err("pop on empty stack while squish".into()),
                    });
                }
                self.store.push(Tree::Branch(v, children));
            }
        }
        Ok(false)
    }
    pub fn get_back(&self) -> &Tree {
        &self.store[0]
    }
    pub fn get_front(&self) -> &Tree {
        &self.store.last().unwrap()
    }

    pub fn is_done(&self) -> bool {
        self.program.is_empty()
    }

    pub fn stack_len(&self) -> usize {
        self.store.len()
    }

    /// Look at the stack from the top: depth 0 = top of stack, depth 1 = one below top.
    /// Returns `None` if depth is out of bounds.
    pub fn peek_tree(&self, depth: usize) -> Option<&Tree> {
        if depth >= self.store.len() {
            return None;
        }
        let idx = self.store.len() - 1 - depth;
        Some(&self.store[idx])
    }

    /// Run the program to completion in one shot.
    pub fn run_all(&mut self) -> Result<(), String> {
        loop {
            match self.step() {
                Ok(true) => return Ok(()),
                Ok(false) => continue,
                Err(e) => return Err(e),
            }
        }
    }

    pub fn parse_program(source: &str) -> Result<Config, String> {
        let sexp_repr = sexp::parse(source).map_err(|e| e.to_string())?;
        let ast = parser::parse(&sexp_repr)?;
        Ok(Config::new(ast))
    }
}
