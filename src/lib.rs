#![allow(non_camel_case_types)]

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
use std::fmt::{Debug};
use std::fs;


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
    pub node_coordinates: Option<Vec<Coord>>,
    pub depots: Option<Vec<usize>>,
    pub demands: Option<Vec<Demand>>,
    pub edges: Option<Vec<EdgeData>>,
    pub fixed_edges: Option<EdgeList>,
    pub display_data: Option<Vec<Coord>>,
    pub tours: Option<Vec<Tour>>,
    pub edge_weights: Option<EdgeWeightData>,
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
pub struct Coord(pub i64, pub N32, pub N32);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Demand(pub u32, pub u32);

pub type Edge = (usize, usize);
pub type EdgeList = Vec<Edge>;
pub type EdgeWeight = u32;
pub type EdgeWeightList = Vec<EdgeWeight>;
pub type EdgeWeightMatrix = Vec<Vec<EdgeWeight>>;
pub type Tour = Vec<usize>;
/// Holds edge information, either in the edge list or adjacency list format.
/// The Adjacency list version is a List of N elements, each of which is a list of
/// connections. Non-connected nodes are still counted as empty lists.
///
pub type Adj = Vec<usize>;
pub type AdjList = Vec<Adj>;
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EdgeData {
    Edge(Edge),
    Adj(Adj),
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EdgeWeightData {
    FUNCTION(Vec<(N32, N32)>),
    FULL_MATRIX(EdgeWeightList),
    UPPER_ROW(EdgeWeightList),
    LOWER_ROW(EdgeWeightList),
    UPPER_DIAG_ROW(EdgeWeightList),
    LOWER_DIAG_ROW(EdgeWeightList),
    UPPER_COL(EdgeWeightList),
    LOWER_COL(EdgeWeightList),
    UPPER_DIAG_COL(EdgeWeightList),
    LOWER_DIAG_COL(EdgeWeightList),
}

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
        (value.trim_end())
    )
);

#[cfg(test)]
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
        Err(_a) => Err(Err::Error((i, nom::error::ErrorKind::ParseTo))),
    })
}
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

fn build_header(
    (name, problem_type, comment, dimension, ewt, capacity, ewf, edf, ddt, nct): (
        Option<String>,
        Option<ProblemType>,
        Option<String>,
        Option<u32>,
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
        problem_type: problem_type.unwrap(),
        comment: comment.unwrap_or_else(|| "".to_string()),
        dimension: dimension.unwrap(),
        capacity,
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
    if input.len() != 1 {
        None
    } else {
        Some(input[0] as usize)
    }
}
fn parse_coord_vec(input: Vec<f32>) -> Option<Coord> {
    if input.len() != 3 {
        None
    } else {
        Some(Coord(input[0] as i64, n32(input[1]), n32(input[2])))
    }
}

fn parse_demand_vec(input: Vec<f32>) -> Option<Demand> {
    if input.len() != 2 {
        None
    } else {
        Some(Demand(input[0] as u32, input[1] as u32))
    }
}

fn parse_edge_vec(input: Vec<f32>) -> Option<Edge> {
    if input.len() != 2 {
        None
    } else {
        Some((input[0] as usize, input[1] as usize))
    }
}

fn parse_tour_vec(input: Vec<f32>) -> Option<Tour> {
    if input.is_empty() {
        None
    } else {
        Some(
            input
                .into_iter()
                .map(|i| i as usize)
                .collect::<Vec<usize>>(),
        )
    }
}

fn parse_weights_vec(input: Vec<f32>) -> Option<EdgeWeightList> {
    if input.is_empty() {
        None
    } else {
        Some(
            input
                .into_iter()
                .map(|i| i as EdgeWeight)
                .collect::<EdgeWeightList>(),
        )
    }
}

fn parse_edgedata_vec(input: Vec<f32>) -> Option<EdgeData> {
    if input.len() != 2 {
        None
    } else {
        Some(EdgeData::Edge((input[0] as usize, input[1] as usize)))
    }
}

fn parse_adjacency_vec(input: Vec<f32>) -> Option<EdgeData> {
    if input.len() < 2 {
        None
    } else {
        Some(EdgeData::Adj(
            input
                .into_iter()
                .map(|i| i as usize)
                .collect::<Vec<usize>>(),
        ))
    }
}

#[test]
fn test_2d_coords() {
    let input = "1 1.0 3.0";
    let input_vec = vec![1.0, 1.0, 3.0];
    assert_eq!(numbers_on_line(input), Ok(("", input_vec.clone())));
    assert_eq!(
        parse_coord_vec(input_vec),
        Some(Coord(1, n32(1.0), n32(3.0)))
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
    //Here we should be building a list of sections that we are expecting based
    //on the header data. At the moment we are making every section optional.
    //So required sections are able to be ommitted and unrelated/nonsensical sections are
    //allowed, which is bad.
    let edge_parser = match header.edge_data_format {
        Some(EdgeDataFormat::ADJ_LIST) => parse_adjacency_vec,
        Some(EdgeDataFormat::EDGE_LIST) => parse_edgedata_vec,
        None => parse_edgedata_vec, //TODO: omit the EDGE_DATA_SECTION if there is no Format for it
    };
    map!(
        input,
        permutation!(
            complete!(call!(get_section, "NODE_COORD_SECTION", parse_coord_vec))?,
            complete!(call!(get_section, "DEPOT_SECTION", parse_depot_vec))?,
            complete!(call!(get_section, "DEMAND_SECTION", parse_demand_vec))?,
            complete!(call!(get_section, "EDGE_DATA_SECTION", edge_parser))?,
            complete!(call!(get_section, "FIXED_EDGES_SECTION", parse_edge_vec))?,
            complete!(call!(get_section, "DISPLAY_DATA_SECTION", parse_coord_vec))?, //TODO make this either 2d or 3d based on DISPLAY_DATA_TYPE
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

fn combine_edge_weights(format: &EdgeWeightFormat, data: &EdgeWeightMatrix) -> EdgeWeightData {
    let mut combined: EdgeWeightList = Vec::new();
    for row in data.iter() {
        combined.extend(row);
    }

    match format {
        //TODO: Figure out what to do with FUNCTION. Probably return an error
        EdgeWeightFormat::FUNCTION => EdgeWeightData::FULL_MATRIX(combined), 
        EdgeWeightFormat::FULL_MATRIX => EdgeWeightData::FULL_MATRIX(combined),
        EdgeWeightFormat::UPPER_ROW => EdgeWeightData::UPPER_ROW(combined),
        EdgeWeightFormat::LOWER_ROW => EdgeWeightData::LOWER_ROW(combined),
        EdgeWeightFormat::UPPER_DIAG_ROW => EdgeWeightData::UPPER_DIAG_ROW(combined),
        EdgeWeightFormat::LOWER_DIAG_ROW => EdgeWeightData::LOWER_DIAG_ROW(combined),
        EdgeWeightFormat::UPPER_COL => EdgeWeightData::UPPER_COL(combined),
        EdgeWeightFormat::LOWER_COL => EdgeWeightData::LOWER_COL(combined),
        EdgeWeightFormat::UPPER_DIAG_COL => EdgeWeightData::UPPER_DIAG_COL(combined),
        EdgeWeightFormat::LOWER_DIAG_COL => EdgeWeightData::LOWER_DIAG_COL(combined),
    }
}
fn build_data(
    header: &TSPLMeta,
    (coords, depots, demands, edge_datas, fixed_edges, ddts, tours, edge_weights): (
        Option<Vec<Coord>>,
        Option<Vec<usize>>,
        Option<Vec<Demand>>,
        Option<Vec<EdgeData>>,
        Option<Vec<Edge>>,
        Option<Vec<Coord>>,
        Option<Vec<Tour>>,
        Option<Vec<EdgeWeightList>>,
    ),
) -> TSPLData {
    TSPLData {
        node_coordinates: coords,
        depots,
        demands,
        display_data: ddts,
        edge_weights: edge_weights
            .map(|e| combine_edge_weights(header.edge_weight_format.as_ref().unwrap(), &e)),
        edges: edge_datas,
        fixed_edges,
        tours,
    }
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

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
                                  Coord(1, n32(565.0), n32(575.0)),
                                  Coord(2, n32(25.0), n32(185.0)),
                                  Coord(3, n32(345.0), n32(750.0)),
        ]);
        t.display_data = Some(vec![
                              Coord(1, n32(8.0), n32(124.0)),
                              Coord(2, n32(125.0), n32(80.0)),
                              Coord(3, n32(97.0), n32(74.0)),
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
    parse_header_perm(input).and_then(|(input, header)| parse_data_section(input, header))
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
