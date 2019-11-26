Tsplib-rs
---------

A Rust parser for TSPLIB problems.

[TSPLIB](http://elib.zib.de/pub/mp-testdata/tsp/tsplib/tsplib.html) is an academic collection of well known Travelling Salesperson Problems and other similar problems.

It is often used in combinatorial problem research, or academic instruction of the TSP. The spec is available [here](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/tsp95.pdf).

The goal of this libarry testing is to be able to parse all of TSPLIB without error.

At the moment it should only be able to parse simple TSP, euclidean programs. There are a few
corners that have been cut in the other implementations.


Features supported
-------------

- [x] All Header information
- [x] Parsing 2d coords
- [x] Parsing 3d coords
- [x] Parsing EUC2D problems
- [ ] Parsing Explicit Edge Weight Matrices
- [ ] Distance functions


Quickstart
---
```
use tsplib::parse_problem;
use std::fs;

let filename = "tests/testdata/berlin52.tsp";
let file_contents = fs::read_to_string(filename).unwrap();
let problem = parse_problem(&file_contents).unwrap().1;
```

Wishlist
--------
A nice little future goal would be able to implement some of the distance functions and be able to transform between
different input problem representations, eg:

* the problem is stated as adjacency list but your program takes an adjacency matrix.
* the problem is stated as a complete, euclidean 2d problem but you want to give your program the pre-calculated adjacency list with the right distances

Another future wishlist item would be to be able to compile the entire TSPLIB library into the crate as a feature flag. So you could have
`tsplib::Examples["tsp/berlin52"]` would give you the berlin52 tsp.

Notes
-----
For the types, my interpretation of the spec is:
* "integers" that are positive numbers are u32, because "The integers are assumed to be represented in 32-bit words."
* "integers" that are indices are represented as `usize`.
* "reals" are exposed as `noisy_float` n64s, which are "non-NaN" floats, which implement Ord and Eq. We use them so we can put them in VecThe spec says "All computations involving floating-point numbers are carried out in double precision arithmetic."

This is built using Nom, the parser combinator library. Unfortunately I don't fully understand how to deal with the errors, as they contain references to the input, and also are a big stack of things, so unfortunately there is not much clean error handling. The goal is that it should be able to parse the example instances or give a rough place the parsing failed.

This was my first Rust project, so feedback is gladly accepted.
