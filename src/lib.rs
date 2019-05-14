use noisy_float::prelude::*;
#[macro_use]
extern crate nom;
use nom::*;
extern crate strum;
#[macro_use]
extern crate strum_macros;

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
named!(name<&str,&str>,
    call!(kv, "NAME")
);
#[test]
fn name_test() {
    assert_eq!(name("NAME: some_name45"), Ok(("", "some_name45")));
    assert_eq!(kv("NAME: some_name45", "NAME"), Ok(("", "some_name45")))
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
