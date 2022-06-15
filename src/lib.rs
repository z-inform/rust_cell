//! An implementation of Conway's "Game of Life" on a really big field with added analyzers for
//! current field state
//!
//! This implementation uses a R*-tree to (hopefully) be able to work with large and long
//! developing patterns

#![allow(dead_code)] //remove after major writing and debugging is finished

pub mod groups;

#[cfg(test)]
mod test;
