Tsplib-rs
---------

An in progress Rust parser for TSPLIB problems.

The goal for testing is to have the tests be "take all the tsplib files and parse them without error". 

A nice little future goal would be able to implement some of the distance functions and be able to transform between
different input problem representations, eg:
* the problem is stated as adjacency list but your program takes an adjacency matrix).
* the problem is stated as a complete, euclidean 2d problem but you want to give your program the pre-calculated adjacency list with the right distances

Another future wishlist item would be to be able to compile the entire TSPLIB library into the crate as a feature flag. So you could have
`tsplib::Examples["tsp/berlin52"]` would give you the berlin52 tsp.

At the moment it should only be able to parse simple TSP, euclidean 2d programs. There are a few
corners that have been cut in the other implementations.


Project Structure
---
The intent is for this to be a library crate.
`main.rs` contains a rudimentary and broken TSP solver. This will be cleaned up soon.

`lib.rs` contains the data types and parsing functionality. 

You'll want to import `parse_full_problem` and it will return an Option<TSPLProblem>.

Notes
-----
Built using Nom, the parser combinator library.

The types are a mess at the moment and need to be sorted out.
In the spect

* "integers" that are indices are represented as `usize`.
* "integers" that are positive numbers are u32.
* "reals" are exposed as `noisy_float` n32s, which are "non-NaN" floats, which implement Ord and Eq.