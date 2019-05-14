use noisy_float::prelude::*;
#[macro_use]
extern crate nom;
use nom::*;

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
//    do_parse!(
//         tag_s!(key) >>
//         tag_s!(":") >>
//         space0 >>
//         value: rest,
//         || rest
// )
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
   do_parse!(
        tag_s!("NAME") >>
        tag_s!(":") >>
        space0 >>
        value: rest >>
        (value)
)
);
#[test]
fn name_test() {
    assert_eq!(name("NAME: some_name45"), Ok(("", "some_name45")));
    assert_eq!(kv("NAME: some_name45", "NAME"), Ok(("", "some_name45")))
}
