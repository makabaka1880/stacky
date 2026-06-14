// Created by Sean L. on Jun. 14.
// Last Updated by Sean L. on Jun. 14.
//
// stacky
// src/parser.rs
//
// Makabaka1880, 2026. All rights reserved.

use std::fmt;

use sexp::{Atom, Sexp};

pub enum Stmt {
    Push(String),
    Pop,
    Squish(String, u64),
}

pub enum Tree {
    Leaf(String),
    Branch(String, Vec<Tree>),
}

impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Leaf(value) => {
                write!(f, "Leaf({value:?})")
            }
            Self::Branch(value, children) => {
                write!(f, "Branch({value:?}, {children:?})")
            }
        }
    }
}

fn parse_ident(sexp: &Sexp) -> Result<String, String> {
    match sexp {
        Sexp::Atom(Atom::S(s)) => Ok(s.clone()),
        other => Err(format!("expected identifier, got {:?}", other)),
    }
}

fn parse_int(sexp: &Sexp) -> Result<i64, String> {
    match sexp {
        Sexp::Atom(Atom::I(n)) => Ok(n.clone()),
        other => Err(format!("expected integer, got {:?}", other)),
    }
}

pub fn parse(raw: &Sexp) -> Result<Vec<Stmt>, String> {
    match raw {
        Sexp::List(sexps) => match sexps.as_slice() {
            [] => Err("expected a statement, got empty list".to_string()),

            [first, rest @ ..] if parse_ident(first) == Ok("begin".to_string()) => rest
                .iter()
                .map(parse)
                .collect::<Result<Vec<Vec<Stmt>>, String>>()
                .map(|vecs| vecs.into_iter().flatten().collect()),

            [cmd] if parse_ident(cmd) == Ok("pop".to_string()) => Ok(vec![Stmt::Pop]),

            [cmd, arg] if parse_ident(cmd) == Ok("push".to_string()) => {
                parse_ident(arg).map(|s| vec![Stmt::Push(s)])
            }

            [cmd, arg1, arg2] if parse_ident(cmd) == Ok("squish".to_string()) => parse_ident(arg1)
                .and_then(|val| {
                    parse_int(arg2).and_then(|n| {
                        if n > 0 {
                            Ok(vec![Stmt::Squish(val, n as u64)])
                        } else {
                            Err(format!("unexpected non-positive squish depth {:?}", n))
                        }
                    })
                }),

            [first, ..] => parse_ident(first)
                .map(|cmd| format!("unexpected command {:?}", cmd))
                .and_then(|err| Err(err))
                .or_else(|e| Err(e)),
        },
        other => Err(format!("expected a statement, got atom {:?}", other)),
    }
}
