use noisy_float::prelude::*;
#[macro_use]
extern crate nom;
use nom::character::complete::*;
use nom::number::complete::*;
use nom::Err::*;
use nom::{Err, IResult};
extern crate strum;
#[macro_use]
extern crate strum_macros;
use std::cmp::PartialEq;
use std::fmt::{Debug, Display};
use std::result::Result;
use strum::IntoEnumIterator;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
struct Coord(i64, N32, N32);

#[derive(Debug, PartialEq, Eq, Clone)]
struct TSPLProblem {
    dimension: i64,
    coords: Vec<Coord>,
    name: String,
    comment: String,
    problem_type: ProblemType,
    capacity: Option<i64>,
    edge_weight_type: EdgeWeightType,
    edge_weight_format: Option<EdgeWeightFormat>,
    edge_data_format: Option<EdgeDataFormat>,
    node_coord_type: NodeCoordType,
    display_data_type: DisplayDataType,
}
#[derive(Debug, PartialEq, Eq, Clone, Display, EnumString, EnumIter)]
enum ProblemType {
    TSP,
    ATSP,
    SOP,
    HCP,
    CVRP,
    TOUR,
}
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Display, EnumString, EnumIter)]
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

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Display, EnumString, EnumIter)]
enum EdgeWeightFormat {
    FUNCTION,
    FULL_MATRIX,
    UPPER_ROW,
    LOWER_ROW,
    UPPER_DIAG_ROW,
    LOWER_DIAG_ROW,
    UPPER_COL,
    LOWER_COL,
    UPPER_DIAG_COL,
    LOWER_DIAG_COL,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Display, EnumString, EnumIter)]
enum EdgeDataFormat {
    EDGE_LIST,
    ADJ_LIST,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Display, EnumString, EnumIter)]
enum NodeCoordType {
    TWOD_COORDS,
    THREED_COORDS,
    NO_COORDS,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Display, EnumString, EnumIter)]
enum DisplayDataType {
    COORDS_DISPLAY,
    TWOD_DISPLAY,
    NO_DISPLAY,
}

named_args!(kv<'a>(key: &'a str)<&'a str, &'a str>,
   do_parse!(
        tag!(key) >>
        tag!(":") >>
        space0 >>
        value: not_line_ending >>
        opt!(line_ending) >>
        (value)
    )
);

fn test_kv<'a, G: Display + Debug + PartialEq + Clone + 'a>(
    kvfunc: fn(&str) -> Result<(&str, G), Err<(&str, nom::error::ErrorKind)>>,
    key: &str,
    value: G,
) {
    let input = format!("{}: {}\n", key, value);
    let output = kvfunc(&input);
    assert_eq!(output, Ok(("", value)));
}

// fn kv_fn<'a>(input: &str, key: &'a str) -> IResult<&'a str, &'a str>,
//    do_parse!(
//         tag!(key) >>
//         tag!(":") >>
//         space0 >>
//         value: not_line_ending >>
//         line_ending >>
//         (value)
//     )
// }

// named_args!(kv_parse<'a>(key: &'a str)<&'a str, &'a T>,
// map_res!(call!(kv, key), str::parse::<T>)
// do_parse!(
//     input,
//     tag!(key)
//         >> tag!(":")
//         >> space0
//         >> value: not_line_ending
//         >> line_ending
//         >> (value)
// ),
// fn many_reads(input:&[u8]) -> IResult<&[u8], Vec<Sequence>> {
fn kv_parse<'a, T>(input: &'a str, key: &'a str) -> IResult<&'a str, T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let kv_out = kv(input, key);
    match kv_out {
        Ok((i, v)) => {
            let parse_res = str::parse::<T>(v);
            match parse_res {
                Ok(tv) => Ok((i, tv)),
                Err(a) => Err(Err::Error((i, nom::error::ErrorKind::ParseTo)))
                // Err(a) => Err(nom::Err::Error(nom::Context::Code(i, ErrorKind::Custom("Parse error"))))
                // Err(a) => nom::Err(Err::Err(a)),
            }
        }
        Err(a) => Err(a),
    }
    // kv_out.map(|(i, v): (&str, &str)| str::parse(v).map(|tv| (i, tv)).unwrap())
    // kv_out.map(|(i, v): (&str, &str)| (i, str::parse(v)))
}
named!(get_name<&str,&str>,
    call!(kv, "NAME")
);
#[test]
fn test_name() {
    assert_eq!(get_name("NAME: some_name45\n"), Ok(("", "some_name45")));
}
named!(get_comment<&str,&str>,
    call!(kv, "COMMENT")
);
#[test]
fn test_comment() {
    let comment = "My favorite TSP instance: Minimum tour around all of my friend's fridges";
    assert_eq!(
        get_comment(&format!("COMMENT: {}\n", comment)),
        Ok(("", comment))
    );
}

fn get_type(input: &str) -> IResult<&str, ProblemType> {
    kv_parse::<ProblemType>(input, "TYPE")
}

#[test]
fn test_type() {
    for ptype in ProblemType::iter() {
        test_kv(get_type, "TYPE", ptype);
    }
}

fn get_dimension(input: &str) -> IResult<&str, i64> {
    kv_parse(input, "DIMENSION")
}
#[test]
fn test_dimension() {
    test_kv(get_dimension, "DIMENSION", 8)
}
#[test]
fn test_dimension_invalid() {
    let output = get_dimension("DIMENSION: ABDC\n");
    assert_eq!(
        output,
        Err((nom::Err::Error(("", nom::error::ErrorKind::ParseTo))))
    )
}

fn get_capacity(input: &str) -> IResult<&str, i64> {
    kv_parse(input, "CAPACITY")
}

#[test]
fn test_capacity() {
    test_kv(get_capacity, "CAPACITY", 8)
}

fn get_edge_weight_type(input: &str) -> IResult<&str, EdgeWeightType> {
    kv_parse(input, "EDGE_WEIGHT_TYPE")
}

#[test]
fn test_edge_weight_type() {
    for ewt in EdgeWeightType::iter() {
        test_kv(get_edge_weight_type, "EDGE_WEIGHT_TYPE", ewt);
    }
}

fn get_edge_weight_format(input: &str) -> IResult<&str, EdgeWeightFormat> {
    kv_parse(input, "EDGE_WEIGHT_FORMAT")
}

#[test]
fn test_edge_weight_format() {
    for ewf in EdgeWeightFormat::iter() {
        test_kv(get_edge_weight_format, "EDGE_WEIGHT_FORMAT", ewf);
    }
}

named!(get_edge_data_format<&str, EdgeDataFormat>,
    map_res!(call!(kv, "EDGE_DATA_FORMAT"), str::parse)
);

#[test]
fn test_edge_data_format() {
    for edf in EdgeDataFormat::iter() {
        test_kv(get_edge_data_format, "EDGE_DATA_FORMAT", edf);
    }
}

named!(get_node_coord_type<&str, NodeCoordType>,
    map_res!(call!(kv, "NODE_COORD_TYPE"), str::parse)
);

#[test]
fn test_node_coord_type() {
    for nct in NodeCoordType::iter() {
        test_kv(get_node_coord_type, "NODE_COORD_TYPE", nct);
    }
}

named!(get_display_data_type<&str, DisplayDataType>,
    map_res!(call!(kv, "DISPLAY_DATA_TYPE"), str::parse)
);

#[test]
fn test_display_data_type() {
    for ddt in DisplayDataType::iter() {
        test_kv(get_display_data_type, "DISPLAY_DATA_TYPE", ddt);
    }
}
// /*
fn build_problem(
    (name, problem_type, comment, dimension, ewt, capacity, ewf, edf, ddt, nct): (
        Option<&str>,
        Option<ProblemType>,
        Option<&str>,
        Option<i64>,
        Option<EdgeWeightType>,
        Option<i64>,
        Option<EdgeWeightFormat>,
        Option<EdgeDataFormat>,
        Option<DisplayDataType>,
        Option<NodeCoordType>,
    ),
) -> TSPLProblem {
    TSPLProblem {
        name: name.unwrap_or("").to_string(),
        problem_type: problem_type.unwrap(),
        comment: comment.unwrap_or("").to_string(),
        dimension: dimension.unwrap(),
        capacity: capacity,
        edge_weight_type: ewt.unwrap_or(EdgeWeightType::EUC_2D),
        coords: Vec::new(),
        edge_data_format: edf,
        edge_weight_format: ewf,
        node_coord_type: nct.unwrap_or(NodeCoordType::NO_COORDS),
        display_data_type: ddt.unwrap_or(DisplayDataType::NO_DISPLAY),
    }
}
fn parse_problem_perm(input: &str) -> IResult<&str, TSPLProblem> {
    map!(
        input,
        dbg_dmp!(permutation!(
            get_name?,
            get_type?,
            get_comment?,
            get_dimension?,
            get_edge_weight_type?,
            get_capacity?,
            get_edge_weight_format?,
            get_edge_data_format?,
            get_display_data_type?,
            opt!(get_node_coord_type)
        )),
        build_problem
    )
}

named!(parse_problem<&str, TSPLProblem>,
    do_parse!(
        name: opt!(get_name) >>
        ptype: opt!(get_type) >>
        comment: opt!(get_comment) >>
        dimension: opt!(get_dimension) >>
        ewt: opt!(get_edge_weight_type) >>
        capacity: opt!(get_capacity) >>
        ewf: opt!(get_edge_weight_format) >>
        edf: opt!(get_edge_data_format) >>
        ddt: opt!(get_display_data_type) >>
        nct: opt!(get_node_coord_type) >>
        (TSPLProblem {
            name:name.unwrap_or("").to_string(),
            problem_type:ptype.unwrap(),
            comment: comment.unwrap_or("").to_string(),
            dimension:dimension.unwrap(),
            capacity: capacity,
            edge_weight_type:ewt.unwrap_or(EdgeWeightType::EUC_2D),
            coords: Vec::new(),
            edge_data_format: edf,
            edge_weight_format: ewf,
            node_coord_type: nct.unwrap_or(NodeCoordType::NO_COORDS),
            display_data_type: ddt.unwrap_or(DisplayDataType::NO_DISPLAY)
        })
    )
);

named!(get_2d_coord<&str, Coord>,
    do_parse!(
        opt!(multispace1) >>
         i: digit1 >>
            space1 >>
         x: float >>
            space1 >>
         y: float >>
         line_ending >>
         (Coord( i.parse().unwrap(), n32(x), n32(y))))
);

#[test]
fn test_2d_coords() {
    let input = " 1 1.0 3.0\n";
    assert_eq!(get_2d_coord(input), Ok(("", Coord(1, n32(1.0), n32(3.0)))));
}
named!(get_node_coord_section<&str, Vec<Coord> >,
    do_parse!(
        tag!("NODE_COORD_SECTION") >>
        line_ending >>
        coords: many1!(get_2d_coord) >>
        opt!(complete!(tag!("EOF\n"))) >>
        (coords)
    )
);

#[test]
fn test_node_coord_section() {
    let ncs = "NODE_COORD_SECTION
1 565.0 575.0
2 25.0 185.0
3 345.0 750.0
EOF
";
    let out = vec![
        Coord(1, n32(565.0), n32(575.0)),
        Coord(2, n32(25.0), n32(185.0)),
        Coord(3, n32(345.0), n32(750.0)),
    ];
    assert_eq!(get_node_coord_section(ncs), Ok(("", out)))
}
#[test]
fn test_parse_problem() {
    let header = "NAME: berlin52
TYPE: TSP
DIMENSION: 52
COMMENT: 52 locations in Berlin (Groetschel)
EDGE_WEIGHT_TYPE: EUC_2D
 ";
    let parsed = TSPLProblem {
        name: String::from("berlin52"),
        problem_type: ProblemType::TSP,
        comment: String::from("52 locations in Berlin (Groetschel)"),
        dimension: 52,
        edge_weight_type: EdgeWeightType::EUC_2D,
        coords: Vec::new(),
        capacity: None,
        display_data_type: DisplayDataType::NO_DISPLAY,
        edge_data_format: None,
        edge_weight_format: None,
        node_coord_type: NodeCoordType::NO_COORDS,
    };
    assert_eq!(parse_problem_perm(header), Ok((" ", parsed)))
}
#[test]
fn test_parse_problem_works_with_missing_data() {
    //atm name, comment, and ege weight type are optional.
    let header = "TYPE: TSP
DIMENSION: 52
EDGE_WEIGHT_TYPE: EUC_2D
 ";
    let parsed = TSPLProblem {
        name: String::from(""),
        problem_type: ProblemType::TSP,
        comment: String::from(""),
        dimension: 52,
        edge_weight_type: EdgeWeightType::EUC_2D,
        coords: Vec::new(),
        capacity: None,
        display_data_type: DisplayDataType::NO_DISPLAY,
        edge_data_format: None,
        edge_weight_format: None,
        node_coord_type: NodeCoordType::NO_COORDS,
    };
    assert_eq!(parse_problem(header), Ok((" ", parsed)))
}
