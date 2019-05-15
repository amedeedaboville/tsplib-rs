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
struct TSPLProblem {
    dimension: i64,
    coords: Vec<Coord>,
    name: String,
    comment: String,
    problem_type: ProblemType,
    edge_weight_type: EdgeWeightType,
}

// enum ProblemMeta {
//     Name(String),
//     Type(ProblemType),
//     Comment(String),
//     Dimension(i64),
//     EWT(EdgeWeightType),
// }
#[derive(Debug, PartialEq, Eq, Clone, EnumString)]
enum ProblemType {
    TSP,
    ATSP,
    SOP,
    HCP,
    CVRP,
    TOUR,
}
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Display, EnumString)]
enum EdgeWeightType {
    EXPLICIT,
    EUC_2D,
    EUC_3D,
    MAX_2D,
    MAX_3D,
    MAN_2D,
    MAN_3D,
    CEIL_2D,
    GEO,
    ATT,
    XRAY1,
    XRAY2,
    SPECIAL,
}

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

named!(get_edge_weight_type<&str, EdgeWeightType>,
    map_res!(call!(kv, "EDGE_WEIGHT_TYPE"), str::parse)
);

#[test]
fn test_edge_weight_type() {
    test_kv(
        get_edge_weight_type,
        "EDGE_WEIGHT_TYPE",
        EdgeWeightType::EUC_2D,
    )
}

named!(parse_problem<&str, TSPLProblem>,
    do_parse!(
        name: opt!(get_name) >>
        ptype: opt!(get_type) >>
        comment: opt!(get_comment) >>
        dimension: opt!(get_dimension) >>
        ewt: opt!(get_edge_weight_type) >>
        (TSPLProblem {
            name:name.unwrap_or("").to_string(),
            problem_type:ptype.unwrap(),
            comment: comment.unwrap_or("").to_string(),
            dimension:dimension.unwrap(),
            edge_weight_type:ewt.unwrap_or(EdgeWeightType::EUC_2D),
            coords: Vec::new(),
        })
    )
);

// fn build_problem(headerInfo: Vec<ProblemMeta>) -> Problem {
//     use ProblemMeta::*;
//     let mut p: Problem = Default::default();
//     for meta in headerInfo.into_iter() {
//         match meta {
//             Name(n) => p.name = n,
//             Type(pt) => p.problem_type = pt,
//             Comment(c) => p.comment = c,
//             Dimension(d) => p.dimension = d,
//             EWT(ewt) => p.edge_weight_type = ewt,
//         };
//     }
//     p
