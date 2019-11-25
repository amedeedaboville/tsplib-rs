Tsplib-rs
---------

An in progress Rust parser for TSPLIB problems and my first Rust probject.

The goal for testing is to have the tests be "take all the tsplib files and parse them without error". 

A nice little future goal would be able to implement some of the distance functions and be able to transform between
different input problem representations, eg:
* the problem is stated as adjacency list but your program takes an adjacency matrix.
* the problem is stated as a complete, euclidean 2d problem but you want to give your program the pre-calculated adjacency list with the right distances

Another future wishlist item would be to be able to compile the entire TSPLIB library into the crate as a feature flag. So you could have
`tsplib::Examples["tsp/berlin52"]` would give you the berlin52 tsp.

At the moment it should only be able to parse simple TSP, euclidean 2d programs. There are a few
corners that have been cut in the other implementations.


Features supported
-------------
[x] - All Header information
[x] - Parsing 2d coords
[x] - Parsing EUC2D problems
[ ] - Parsing 3d coords

Quickstart
---
```
use tsplib::parse_whole_problem;
use std::fs;

let filename = "tests/testdata/berlin52.tsp";
let file_contents = fs::read_to_string(filename).unwrap();
let problem = parse_whole_problem(&file_contents).unwrap().1;
```

Notes
-----
Built using Nom, the parser combinator library.

The types are a mess at the moment and need to be sorted out.
My interpretation of the spec is:

* "integers" that are indices are represented as `usize`.
* "integers" that are positive numbers are u32.
* "reals" are exposed as `noisy_float` n32s, which are "non-NaN" floats, which implement Ord and Eq.