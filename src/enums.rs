use noisy_float::prelude::*;
use std::cmp::PartialEq;
use std::fmt::Debug;

//We break down the parsing into two steps, parsing the header and then
//the problem body based on the metadata in the header:
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TSPLProblem {
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
    pub edge_weights: Option<EdgeWeightList>,
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
pub enum Coord {
    Coord2(i64, N64, N64),
    Coord3(i64, N64, N64, N64),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Demand(pub u32, pub u32);

pub type Tour = Vec<usize>;
pub type Edge = (usize, usize);
pub type EdgeList = Vec<Edge>;
pub type EdgeWeight = u32;
pub type EdgeWeightList = Vec<EdgeWeight>;
pub type Adj = Vec<usize>;

/// Holds edge information, either in the edge list or adjacency list format.
/// The adjacency list version is a Vec of N elements, each of which is a list of
/// connections. Non-connected nodes are still counted as empty lists.
/// TSPLData has a Vec<EdgeData>.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EdgeData {
    Edge(Edge),
    Adj(Adj),
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
