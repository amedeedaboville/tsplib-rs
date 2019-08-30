TSPLIB-RS
---------

An attempt at a Rust parser for TSPLIB problems.

Currently still in progress.

The goal is to have the tests be "take all the tsplib files and parse them without error". 

Built using Nom, the parser combinator library.

Project Structure
---
The intent is for this to be a library crate.
`main.rs` contains a rudimentary and broken TSP solver. This will be cleaned up soon.

`lib.rs` contains the data types and parsing functionality. 

You'll want to import `parse_full_problem` and it will return an Option<TSPLProblem>.