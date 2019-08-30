use noisy_float::prelude::*;
#[macro_use]
extern crate nom;
use nom::character::complete::*;
use nom::number::complete::*;
use nom::{Err, IResult};
extern crate strum;
#[macro_use]
extern crate strum_macros;
use std::cmp::PartialEq;
use std::fmt::{Debug, Display};
//use std::result::Result;
use strum::IntoEnumIterator;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
struct Coord(i64, N32, N32);

struct FullProblem {
    header: TSPLMeta,
    // data: TSPLData,
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct TSPLData {
    node_coordinates: Vec<Coord>,
    depots: Vec<i64>,
    demands: Vec<i64>,
    edges: EdgeData,
    fixed_edges: EdgeList,
    display_data: Vec<(N32, N32)>,
    tours: Vec<Vec<usize>>,
    edge_weights: EdgeWeightData,
}
type EdgeList = Vec<(usize, usize)>;
type EdgeWeights = Vec<Vec<i64>>;
/// Holds edge information, either in the edge list or adjacency list format.
/// The Adjacency list version is a List of N elements, each of which is a list of
/// connections. Non-connected nodes are still counted as empty lists.
///
#[derive(Debug, PartialEq, Eq, Clone)]
enum EdgeData {
    EdgeList(EdgeList),
    AdjList(Vec<Vec<usize>>),
}
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone)]
enum EdgeWeightData {
    FUNCTION(Vec<(N32, N32)>),
    FULL_MATRIX(EdgeWeights),
    UPPER_ROW(EdgeWeights),
    LOWER_ROW(EdgeWeights),
    UPPER_DIAG_ROW(EdgeWeights),
    LOWER_DIAG_ROW(EdgeWeights),
    UPPER_COL(EdgeWeights),
    LOWER_COL(EdgeWeights),
    UPPER_DIAG_COL(EdgeWeights),
    LOWER_DIAG_COL(EdgeWeights),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct TSPLMeta {
    name: String,
    problem_type: ProblemType,
    comment: String,
    dimension: i64,
    capacity: Option<u64>,
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

fn ints_on_line(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list!(input, space1, digit1)
}
#[test]
fn test_ints_on_line() {
    assert_eq!(ints_on_line("1 2 3 4"), Ok(("", vec!["1", "2", "3", "4"])));
}

fn numbers_on_line(input: &str) -> IResult<&str, Vec<f32>> {
    separated_list!(input, space1, float)
}
#[test]
fn test_numbers_on_line() {
    assert_eq!(
        numbers_on_line("1.2 2 3 4"),
        Ok(("", vec![1.2, 2.0, 3.0, 4.0]))
    );
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

fn test_kv<'a, G: std::str::FromStr + Display + Debug + PartialEq + Clone + 'a>(
    key: &str,
    value: G,
) {
    let input = format!("{}: {}\n", key, value);
    let output: IResult<&str, G> = kv_parse(&input, key);
    assert_eq!(output, Ok(("", value)));
}

fn kv_parse<'a, T>(input: &'a str, key: &'a str) -> IResult<&'a str, T>
where
    T: std::str::FromStr,
{
    kv(input, key).and_then(|(i, v)| match str::parse::<T>(v) {
        Ok(tv) => Ok((i, tv)),
        Err(a) => Err(Err::Error((i, nom::error::ErrorKind::ParseTo))),
    })
}
#[test]
fn test_name() {
    test_kv("NAME", "some_name".to_string());
}
#[test]
fn test_comment() {
    let comment =
        "My favorite TSP instance: Minimum tour around all of my friend's fridges".to_string();
    test_kv("COMMENT", comment);
}

#[test]
fn test_edge_weight_type() {
    for ewt in EdgeWeightType::iter() {
        test_kv("EDGE_WEIGHT_TYPE", ewt);
    }
}
#[test]
fn test_edge_weight_format() {
    for ewf in EdgeWeightFormat::iter() {
        test_kv("EDGE_WEIGHT_FORMAT", ewf);
    }
}

#[test]
fn test_edge_data_format() {
    for edf in EdgeDataFormat::iter() {
        test_kv("EDGE_DATA_FORMAT", edf);
    }
}

#[test]
fn test_node_coord_type() {
    for nct in NodeCoordType::iter() {
        test_kv("NODE_COORD_TYPE", nct);
    }
}
#[test]
fn test_display_data_type() {
    for ddt in DisplayDataType::iter() {
        test_kv("DISPLAY_DATA_TYPE", ddt);
    }
}

fn build_header(
    (name, problem_type, comment, dimension, ewt, capacity, ewf, edf, ddt, nct): (
        Option<String>,
        Option<ProblemType>,
        Option<String>,
        Option<i64>,
        Option<EdgeWeightType>,
        Option<u64>,
        Option<EdgeWeightFormat>,
        Option<EdgeDataFormat>,
        Option<DisplayDataType>,
        Option<NodeCoordType>,
    ),
) -> TSPLMeta {
    TSPLMeta {
        name: name.unwrap_or("".to_string()),
        problem_type: problem_type.unwrap(),
        comment: comment.unwrap_or("".to_string()),
        dimension: dimension.unwrap(),
        capacity: capacity,
        edge_weight_type: ewt.unwrap_or(EdgeWeightType::EUC_2D),
        edge_data_format: edf,
        edge_weight_format: ewf,
        node_coord_type: nct.unwrap_or(NodeCoordType::NO_COORDS),
        display_data_type: ddt.unwrap_or(DisplayDataType::NO_DISPLAY),
    }
}
fn parse_header_perm(input: &str) -> IResult<&str, TSPLMeta> {
    map!(
        input,
        permutation!(
            call!(kv_parse, "NAME")?,
            call!(kv_parse, "TYPE")?,
            call!(kv_parse, "COMMENT")?,
            call!(kv_parse, "DIMENSION")?,
            call!(kv_parse, "EDGE_WEIGHT_TYPE")?,
            call!(kv_parse, "CAPACITY")?,
            call!(kv_parse, "EDGE_WEIGHT_FORMAT")?,
            call!(kv_parse, "EDGE_DATA_FORMAT")?,
            call!(kv_parse, "DISPLAY_DATA_TYPE")?,
            opt!(call!(kv_parse, "NODE_COORD_TYPE"))
        ),
        build_header
    )
}

fn get_section<'a, T>(
    input: &'a str,
    section_title: &'a str,
    line_parser: fn(&'a str) -> IResult<&'a str, T>,
) -> IResult<&'a str, Vec<T>> {
    do_parse!(
        input,
        tag!(section_title)
            >> payload: separated_list!(line_ending, line_parser)
            >> line_ending
            >> opt!(complete!(tag!("EOF\n")))
            >> (payload)
    )
}
fn parse_node_coord_vec(input: Vec<f32>) -> Option<Coord> {
    if input.len() != 3 {
        None
    } else {
        Some(Coord(input[0] as i64, n32(input[1]), n32(input[2])))
    }
}

fn get_2d_coord(input: &str) -> IResult<&str, Coord> {
    map_opt!(input, numbers_on_line, parse_node_coord_vec)
}

#[test]
fn test_2d_coords() {
    let input = "1 1.0 3.0";
    assert_eq!(numbers_on_line(input), Ok(("", vec![1.0, 1.0, 3.0])));
    assert_eq!(get_2d_coord(input), Ok(("", Coord(1, n32(1.0), n32(3.0)))));
    let input2 = "1 1.0 3.0";
    assert_eq!(get_2d_coord(input), Ok(("", Coord(1, n32(1.0), n32(3.0)))));
}

fn get_node_coord_section(input: &str) -> IResult<&str, Vec<Coord>> {
    get_section(input, "NODE_COORD_SECTION", get_2d_coord)
}
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
fn get_edge_weight_section(input: &str) -> IResult<&str, Vec<Coord>> {
    get_section(input, "EDGE_WEIGHT_SECTION", get_2d_coord)
}
#[test]
fn test_parse_problem() {
    let header = "NAME: berlin52
TYPE: TSP
DIMENSION: 52
COMMENT: 52 locations in Berlin (Groetschel)
EDGE_WEIGHT_TYPE: EUC_2D
 ";
    let parsed = TSPLMeta {
        name: String::from("berlin52"),
        problem_type: ProblemType::TSP,
        comment: String::from("52 locations in Berlin (Groetschel)"),
        dimension: 52,
        edge_weight_type: EdgeWeightType::EUC_2D,
        capacity: None,
        display_data_type: DisplayDataType::NO_DISPLAY,
        edge_data_format: None,
        edge_weight_format: None,
        node_coord_type: NodeCoordType::NO_COORDS,
    };
    assert_eq!(parse_header_perm(header), Ok((" ", parsed)))
}
#[test]
fn test_parse_problem_works_with_missing_data() {
    //atm name, comment, and ege weight type are optional.
    let header = "TYPE: TSP
DIMENSION: 52
EDGE_WEIGHT_TYPE: EUC_2D
 ";
    let parsed = TSPLMeta {
        name: String::from(""),
        problem_type: ProblemType::TSP,
        comment: String::from(""),
        dimension: 52,
        edge_weight_type: EdgeWeightType::EUC_2D,
        capacity: None,
        display_data_type: DisplayDataType::NO_DISPLAY,
        edge_data_format: None,
        edge_weight_format: None,
        node_coord_type: NodeCoordType::NO_COORDS,
    };
    assert_eq!(parse_header_perm(header), Ok((" ", parsed)))
}

fn parse_data_section<'a>(input: &'a str, header: TSPLMeta) -> IResult<&'a str, FullProblem> {
    let coord_parser = get_2d_coord;
    map!(
        input,
        permutation!(
            call!(get_section, "DEPOT_SECTION", digit1)?,
            call!(get_section, "NODE_COORD_SECTION", get_2d_coord)
        ),
        |x| {
            FullProblem {
                header: header.clone(),
                // data: TSPLData {},
            }
        }
    )
}
// fn build_problem_with_data(
//     (name, problem_type, comment, dimension, ewt, capacity, ewf, edf, ddt, nct): (
//         Option<String>,
//         Option<ProblemType>,
//         Option<String>,
//         Option<i64>,
//         Option<EdgeWeightType>,
//         Option<i64>,
//         Option<EdgeWeightFormat>,
//         Option<EdgeDataFormat>,
//         Option<DisplayDataType>,
//         Option<NodeCoordType>,
//     ),
// ) -> TSPLProblem {
//     TSPLProblem {
//         name: name.unwrap_or("".to_string()),
//         problem_type: problem_type.unwrap(),
//         comment: comment.unwrap_or("".to_string()),
//         dimension: dimension.unwrap(),
//         capacity: capacity,
//         edge_weight_type: ewt.unwrap_or(EdgeWeightType::EUC_2D),
//         edge_data_format: edf,
//         edge_weight_format: ewf,
//         node_coord_type: nct.unwrap_or(NodeCoordType::NO_COORDS),
//         display_data_type: ddt.unwrap_or(DisplayDataType::NO_DISPLAY),
//     }
// }

fn parse_whole_problem<'a>(input: &'a str) -> IResult<&'a str, FullProblem> {
    parse_header_perm(input).and_then(|(input, header)| parse_data_section(input, header))
}
