#![allow(non_camel_case_types)]

use noisy_float::prelude::*;
#[macro_use]
extern crate nom;
use nom::character::complete::{line_ending, multispace1, not_line_ending, space0, space1};
use nom::number::complete::float;
use nom::{Err, IResult};
extern crate strum;
#[macro_use]
extern crate strum_macros;
use std::cmp::PartialEq;
use std::fmt::Debug;
use std::fs;
use strum::IntoEnumIterator;

//We break down the parsing into two steps, parsing the header and then
//the problem body based on the metadata in the header:
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FullProblem {
    pub header: TSPLMeta,
    pub data: TSPLData,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TSPLMeta {
    pub name: String,
    pub problem_type: ProblemType,
    pub comment: String,
    pub dimension: u32,
    pub capacity: Option<u32>,
    pub edge_weight_type: EdgeWeightType,
    pub edge_weight_format: Option<EdgeWeightFormat>,
    pub edge_data_format: Option<EdgeDataFormat>,
    pub node_coord_type: NodeCoordType,
    pub display_data_type: DisplayDataType,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TSPLData {
    pub node_coordinates: Option<Vec<Coord2>>,
    pub depots: Option<Vec<usize>>,
    pub demands: Option<Vec<Demand>>,
    pub edges: Option<Vec<EdgeData>>,
    pub fixed_edges: Option<EdgeList>,
    pub display_data: Option<Vec<Coord2>>,
    pub tours: Option<Vec<Tour>>,
    pub edge_weights: Option<EdgeWeightMatrix>,
}
impl TSPLData {
    pub fn empty() -> TSPLData {
        TSPLData {
            node_coordinates: None,
            depots: None,
            demands: None,
            edges: None,
            fixed_edges: None,
            display_data: None,
            tours: None,
            edge_weights: None,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Coord2(pub i64, pub N32, pub N32);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Coord3(pub i64, pub N32, pub N32, pub N32);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Demand(pub u32, pub u32);

pub type Edge = (usize, usize);
pub type EdgeList = Vec<Edge>;
pub type EdgeWeight = u32;
pub type EdgeWeightList = Vec<EdgeWeight>;
pub type EdgeWeightMatrix = Vec<Vec<EdgeWeight>>;
pub type Tour = Vec<usize>;

/// Holds edge information, either in the edge list or adjacency list format.
/// The adjacency list version is a Vec of N elements, each of which is a list of
/// connections. Non-connected nodes are still counted as empty lists.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EdgeData {
    Edge(Edge),
    Adj(Adj),
}
pub type Adj = Vec<usize>;
pub type AdjList = Vec<Adj>;

#[derive(Debug, PartialEq, Eq, Clone, Display, EnumString, EnumIter)]
pub enum ProblemType {
    TSP,
    ATSP,
    SOP,
    HCP,
    CVRP,
    TOUR,
}
#[derive(Debug, PartialEq, Eq, Clone, Display, EnumString, EnumIter)]
pub enum EdgeWeightType {
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

#[derive(Debug, PartialEq, Eq, Clone, Display, EnumString, EnumIter)]
pub enum EdgeWeightFormat {
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

#[derive(Debug, PartialEq, Eq, Clone, Display, EnumString, EnumIter)]
pub enum EdgeDataFormat {
    EDGE_LIST,
    ADJ_LIST,
}

#[derive(Debug, PartialEq, Eq, Clone, Display, EnumString, EnumIter)]
pub enum NodeCoordType {
    TWOD_COORDS,
    THREED_COORDS,
    NO_COORDS,
}

#[derive(Debug, PartialEq, Eq, Clone, Display, EnumString, EnumIter)]
pub enum DisplayDataType {
    COORDS_DISPLAY,
    TWOD_DISPLAY,
    NO_DISPLAY,
}

//Gives us a parser called kv() that takes a key to look for, and will return
//the value in KEY: VALUE
named_args!(kv<'a>(key: &'a str)<&'a str, &'a str>,
   do_parse!(
        tag!(key) >>
        tag!(":") >>
        space0 >>
        value: not_line_ending >>
        opt!(line_ending) >>
        (value.trim_end())
    )
);

fn kv_parse<'a, T>(input: &'a str, key: &'a str) -> IResult<&'a str, T>
where
    T: std::str::FromStr,
{
    kv(input, key).and_then(|(i, v)| match str::parse::<T>(v) {
        Ok(tv) => Ok((i, tv)),
        Err(_a) => Err(Err::Error((i, nom::error::ErrorKind::ParseTo))),
    })
}

//TSPLIB defines all of these key values in its header, and they can be set
//in any order.
//Cool note: the way we map! the result of permutation! to build_header tells each
//kv_parse! the type of each key:value (eg ProblemType), and calls its
//the right string -> Data type conversion automatically.
fn parse_header(input: &str) -> IResult<&str, TSPLMeta> {
    map!(
        input,
        permutation!(
            call!(kv_parse, "NAME")?,
            call!(kv_parse, "TYPE"),
            call!(kv_parse, "COMMENT")?,
            call!(kv_parse, "DIMENSION"),
            call!(kv_parse, "EDGE_WEIGHT_TYPE")?,
            call!(kv_parse, "CAPACITY")?,
            call!(kv_parse, "EDGE_WEIGHT_FORMAT")?,
            call!(kv_parse, "EDGE_DATA_FORMAT")?,
            call!(kv_parse, "DISPLAY_DATA_TYPE")?,
            call!(kv_parse, "NODE_COORD_TYPE")?
        ),
        build_header
    )
}

fn build_header(
    (name, problem_type, comment, dimension, ewt, capacity, ewf, edf, ddt, nct): (
        Option<String>,
        ProblemType,
        Option<String>,
        u32,
        Option<EdgeWeightType>,
        Option<u32>,
        Option<EdgeWeightFormat>,
        Option<EdgeDataFormat>,
        Option<DisplayDataType>,
        Option<NodeCoordType>,
    ),
) -> TSPLMeta {
    TSPLMeta {
        name: name.unwrap_or_else(|| "".to_string()),
        problem_type,
        comment: comment.unwrap_or_else(|| "".to_string()),
        dimension,
        capacity,
        edge_weight_type: ewt.unwrap_or(EdgeWeightType::EUC_2D),
        edge_data_format: edf,
        edge_weight_format: ewf,
        node_coord_type: nct.unwrap_or(NodeCoordType::NO_COORDS),
        display_data_type: ddt.unwrap_or(DisplayDataType::NO_DISPLAY),
    }
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

fn get_section<'a, T>(
    input: &'a str,
    section_title: &'a str,
    line_parser: fn(Vec<f32>) -> Option<T>,
) -> IResult<&'a str, Vec<T>>
where
    T: std::fmt::Debug,
{
    let out = do_parse!(
        input,
        tag!(section_title)
            >> line_ending
            >> space0
            >> payload: separated_list!(multispace1, map_opt!(numbers_on_line, line_parser))
            >> space0
            >> opt!(line_ending)
            >> opt!(complete!(tag!("EOF\n")))
            >> (payload)
    );
    println!(
        "Get section for title {:}, returning {:?}",
        section_title, out
    );
    out
}
fn parse_depot_vec(input: Vec<f32>) -> Option<usize> {
    match input.len() {
        1 => Some(input[0] as usize),
        _ => None,
    }
}
fn parse_coord2_vec(input: Vec<f32>) -> Option<Coord2> {
    match input.len() {
        3 => Some(Coord2(input[0] as i64, n32(input[1]), n32(input[2]))),
        _ => None,
    }
}

fn parse_coord3_vec(input: Vec<f32>) -> Option<Coord3> {
    match input.len() {
        4 => Some(Coord3(
            input[0] as i64,
            n32(input[1]),
            n32(input[2]),
            n32(input[3]),
        )),
        _ => None,
    }
}

fn parse_demand_vec(input: Vec<f32>) -> Option<Demand> {
    match input.len() {
        2 => Some(Demand(input[0] as u32, input[1] as u32)),
        _ => None,
    }
}

fn parse_edge_vec(input: Vec<f32>) -> Option<Edge> {
    match input.len() {
        2 => Some((input[0] as usize, input[1] as usize)),
        _ => None,
    }
}

fn parse_tour_vec(input: Vec<f32>) -> Option<Tour> {
    match input.len() {
        0 => None,
        _ => Some(
            input
                .into_iter()
                .map(|i| i as usize)
                .collect::<Vec<usize>>(),
        ),
    }
}

fn parse_weights_vec(input: Vec<f32>) -> Option<EdgeWeightList> {
    match input.len() {
        0 => None,
        _ => Some(
            input
                .into_iter()
                .map(|i| i as EdgeWeight)
                .collect::<EdgeWeightList>(),
        ),
    }
}

fn parse_edgedata_vec(input: Vec<f32>) -> Option<EdgeData> {
    match input.len() {
        2 => Some(EdgeData::Edge((input[0] as usize, input[1] as usize))),
        _ => None,
    }
}

fn parse_adjacency_vec(input: Vec<f32>) -> Option<EdgeData> {
    match input.len() {
        len if len < 2 => None,
        _ => Some(EdgeData::Adj(
            input
                .into_iter()
                .map(|i| i as usize)
                .collect::<Vec<usize>>(),
        )),
    }
}

#[test]
fn test_2d_coords() {
    let input = "1 1.0 3.0";
    let input_vec = vec![1.0, 1.0, 3.0];
    assert_eq!(numbers_on_line(input), Ok(("", input_vec.clone())));
    assert_eq!(
        parse_coord2_vec(input_vec),
        Some(Coord2(1, n32(1.0), n32(3.0)))
    );
}

#[test]
fn test_parse_meta() {
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
    assert_eq!(parse_header(header), Ok((" ", parsed)))
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
    assert_eq!(parse_header(header), Ok((" ", parsed)))
}

fn parse_data_section<'a>(input: &'a str, header: TSPLMeta) -> IResult<&'a str, FullProblem> {
    //Here we should be building a list of sections that we are expecting based
    //on the header data. At the moment we are making every section optional.
    //So required sections are able to be omitted and unrelated/nonsensical sections are
    //allowed.
    //Also, the header is fully cloned, because I couldn't figure out the lifetimes.
    let edge_parser = match header.edge_data_format {
        Some(EdgeDataFormat::ADJ_LIST) => parse_adjacency_vec,
        Some(EdgeDataFormat::EDGE_LIST) => parse_edgedata_vec,
        None => parse_edgedata_vec, //TODO: omit the EDGE_DATA_SECTION if there is no Format for it
    };
    //This doesn't work because the return types parse_coord2 and parse3 are different.
    //So we either make COORD an enum variant, which I wouldn't like, because it feels like it would
    //allow polymorphic lists.
    // let coord_parser = match header.node_coord_type {
    //     NodeCoordType::TWOD_COORDS => parse_coord2_vec,
    //     NodeCoordType::THREED_COORDS => parse_coord3_vec,
    //     NodeCoordType::NO_COORDS => parse_coord2_vec, //TODO: same here, omit looking for node_coords
    // };
    map!(
        input,
        permutation!(
            complete!(call!(get_section, "NODE_COORD_SECTION", parse_coord2_vec))?,
            complete!(call!(get_section, "DEPOT_SECTION", parse_depot_vec))?,
            complete!(call!(get_section, "DEMAND_SECTION", parse_demand_vec))?,
            complete!(call!(get_section, "EDGE_DATA_SECTION", edge_parser))?,
            complete!(call!(get_section, "FIXED_EDGES_SECTION", parse_edge_vec))?,
            complete!(call!(get_section, "DISPLAY_DATA_SECTION", parse_coord2_vec))?, //TODO make this either 2d or 3d based on DISPLAY_DATA_TYPE
            complete!(call!(get_section, "TOUR_SECTION", parse_tour_vec))?,
            complete!(call!(get_section, "EDGE_WEIGHT_SECTION", parse_weights_vec))?
        ),
        |x| {
            println!("Parsed successfully got {:?}", x);
            FullProblem {
                header: header.clone(),
                data: build_data(&header, x),
            }
        }
    )
}

fn build_data(
    header: &TSPLMeta,
    (node_coordinates, depots, demands, edges, fixed_edges, display_data, tours, edge_weights): (
        Option<Vec<Coord2>>,
        Option<Vec<usize>>,
        Option<Vec<Demand>>,
        Option<Vec<EdgeData>>,
        Option<Vec<Edge>>,
        Option<Vec<Coord2>>,
        Option<Vec<Tour>>,
        Option<Vec<EdgeWeightList>>,
    ),
) -> TSPLData {
    TSPLData {
        node_coordinates,
        depots,
        demands,
        display_data,
        edge_weights,
        edges,
        fixed_edges,
        tours,
    }
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::fmt::{Debug, Display};

    fn test_kv<'a, G: std::str::FromStr + Display + Debug + PartialEq + Clone + 'a>(
        key: &str,
        value: G,
    ) {
        let input = format!("{}: {}\n", key, value);
        let output: IResult<&str, G> = kv_parse(&input, key);
        assert_eq!(output, Ok(("", value)));
    }

    //Will give kv_parse a key and a value separated by a : and see if it roundtrips ok.
    #[test]
    fn test_some_kvs() {
        test_kv("NAME", "some_name".to_string());
        test_kv(
            "COMMENT",
            "My favorite TSP instance: Minimum tour around all of my friend's fridges".to_string(),
        );
    }

    #[test]
    fn test_iterable_enums() {
        for ewt in EdgeWeightType::iter() {
            test_kv("EDGE_WEIGHT_TYPE", ewt);
        }
        for ewf in EdgeWeightFormat::iter() {
            test_kv("EDGE_WEIGHT_FORMAT", ewf);
        }
        for edf in EdgeDataFormat::iter() {
            test_kv("EDGE_DATA_FORMAT", edf);
        }
        for nct in NodeCoordType::iter() {
            test_kv("NODE_COORD_TYPE", nct);
        }
        for ddt in DisplayDataType::iter() {
            test_kv("DISPLAY_DATA_TYPE", ddt);
        }
    }

    #[test]
    fn test_parse_data_section() {
        let header = TSPLMeta {
            comment: "".to_string(),
            dimension: 0,
            display_data_type: DisplayDataType::NO_DISPLAY,
            name: "TEST".to_string(),
            problem_type: ProblemType::TSP,
            capacity: None,
            edge_weight_type: EdgeWeightType::EUC_2D,
            edge_weight_format: None,
            edge_data_format: None,
            node_coord_type: NodeCoordType::TWOD_COORDS,
        };
        let ncs = "NODE_COORD_SECTION
1 565.0 575.0
2 25.0 185.0
3 345.0 750.0
DISPLAY_DATA_SECTION
1 8.0 124.0
2 125.0 80.0
3 97.0 74.0
EOF
";

        let mut t = TSPLData::empty();
        t.node_coordinates = Some(vec![
            Coord2(1, n32(565.0), n32(575.0)),
            Coord2(2, n32(25.0), n32(185.0)),
            Coord2(3, n32(345.0), n32(750.0)),
        ]);
        t.display_data = Some(vec![
            Coord2(1, n32(8.0), n32(124.0)),
            Coord2(2, n32(125.0), n32(80.0)),
            Coord2(3, n32(97.0), n32(74.0)),
        ]);
        assert_eq!(
            parse_data_section(ncs, header.clone()),
            Ok((
                "",
                FullProblem {
                    header: header.clone(),
                    data: t,
                }
            ))
        );
    }
}

pub fn parse_whole_problem<'a>(input: &'a str) -> IResult<&'a str, FullProblem> {
    parse_header(input).and_then(|(input, header)| parse_data_section(input, header))
}

fn parse_whole_problem_opt(input: String) -> Option<FullProblem> {
    let r_tuple = parse_whole_problem(&input);
    r_tuple.map(|x| x.1).ok()
}

pub fn parse_file_opt(filename: &str) -> Option<FullProblem> {
    fs::read_to_string(filename)
        .ok()
        .and_then(parse_whole_problem_opt)
}
