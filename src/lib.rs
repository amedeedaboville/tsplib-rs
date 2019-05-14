use noisy_float::prelude::*;
#[macro_use]
extern crate nom;
use nom::*;
extern crate strum;
#[macro_use]
extern crate strum_macros;
use std::cmp::{Eq, PartialEq};
use std::fmt::{Debug, Display};
use std::result::Result;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Coord {
    i: usize,
    x: N32,
    y: N32,
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct Problem {
    n: usize,
    coords: Vec<Coord>,
    comment: String,
    edge_weights: String,
}

#[derive(Debug, PartialEq, Eq, EnumString)]
enum ProblemType {
    TSP,
    ATSP,
    SOP,
    HCP,
    CVRP,
    TOUR,
}

// named!(node_coords_section<&str,&Vec<Coord>,
//    chain!(
//     tag_s!("NODE_COORD_SECTION\n") ~
//     many1(node_coords)
//    )
// );
// named!(node_coords<&str,&Coord>,
//     i, x, y = chain!(number!(), float!(), float!());
//     Coord { i , x, y}
// );

named_args!(kv<'a>(key: &'a str)<&'a str, &'a str>,
   do_parse!(
        tag_s!(key) >>
        tag_s!(":") >>
        space0 >>
        value: rest >>
        (value)
)
);

fn test_kv<'a, G: Display + Debug + PartialEq + Clone + 'a>(
    kvfunc: fn(&str) -> Result<(&str, G), Err<&str>>,
    key: &str,
    value: G,
) {
    let input = format!("{}: {}", key, value);
    let output = kvfunc(&input);
    assert_eq!(output, Ok(("", value)));
}

named!(get_name<&str,&str>,
    call!(kv, "NAME")
);
#[test]
fn test_name() {
    assert_eq!(get_name("NAME: some_name45"), Ok(("", "some_name45")));
    // test_kv(get_name, "NAME", "some_name");
}
named!(get_comment<&str,&str>,
    call!(kv, "COMMENT")
);
#[test]
fn test_comment() {
    let comment = "My favorite TSP instance: Minimum tour around all of my friend's fridges";
    assert_eq!(
        get_comment(&format!("COMMENT: {}", comment)),
        Ok(("", comment))
    );
}

named!(get_type<&str, ProblemType>,
    map_res!(call!(kv, "TYPE"), str::parse)
);

#[test]
fn test_type() {
    assert_eq!(get_type("TYPE: TSP"), Ok(("", ProblemType::TSP)));
    assert_eq!(get_type("TYPE: ATSP"), Ok(("", ProblemType::ATSP)));
    assert_eq!(get_type("TYPE: SOP"), Ok(("", ProblemType::SOP)));
    assert_eq!(get_type("TYPE: HCP"), Ok(("", ProblemType::HCP)));
    assert_eq!(get_type("TYPE: CVRP"), Ok(("", ProblemType::CVRP)));
    assert_eq!(get_type("TYPE: TOUR"), Ok(("", ProblemType::TOUR)));
}

named!(get_dimension<&str, i64>,
    map_res!(call!(kv, "DIMENSION"), str::parse::<i64>)
);
#[test]
fn test_dimension() {
    test_kv(get_dimension, "DIMENSION", 8)
}

named!(get_capacity<&str, i64>,
    map_res!(call!(kv, "CAPACITY"), str::parse)
);

#[test]
fn test_capacity() {
    test_kv(get_capacity, "CAPACITY", 8)
}
