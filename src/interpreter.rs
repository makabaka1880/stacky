// Created by Sean L. on Jun. 14.
// Last Updated by Sean L. on Jun. 14.
//
// stacky
// src/interpreter.rs
//
// Makabaka1880, 2026. All rights reserved.

use crate::parser::{Stmt, Tree};

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
}
