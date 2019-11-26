#![allow(non_camel_case_types)]
use noisy_float::prelude::*;
use std::cmp::PartialEq;
use std::fmt::Debug;

//We break down the parsing into two steps, parsing the header and then
//the problem body based on the metadata in the header:

///A TSPLIB Instance
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TSPLProblem {
    pub header: TSPLMeta,
    pub data: TSPLData,
}

///Header information for the problem instance
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TSPLMeta {
    ///Identifies the data file.
    pub name: String,
    ///Specifies the type of the problem instance.
    pub problem_type: ProblemType,
    ///Additional comments (usually the name of the contributor or creator of the problem instance is given here).
    pub comment: String,
    ///For a TSP or ATSP, the dimension is the number of its nodes. For a CVRP, it is the total number of nodes and depots. For a TOUR file it is the dimension of the corresponding problem.
    pub dimension: u32,
    ///Specifies the truck capacity in a CVRP.
    pub capacity: Option<u32>,
    ///Specifies how the edge weights (or distances) are given.
    pub edge_weight_type: EdgeWeightType,
    ///Describes the format of the edge weights if they are given explicitly.
    pub edge_weight_format: Option<EdgeWeightFormat>,
    ///Describes the format in which the edges of a graph are given, if the graph is not complete.
    pub edge_data_format: Option<EdgeDataFormat>,
    ///Specifies whether coordinates are associated with each node (which, for example may be used for either graphical display or distance computations).
    pub node_coord_type: NodeCoordType,
    ///Specifies how a graphical display of the nodes can be obtained.
    pub display_data_type: DisplayDataType,
}

///Problem instance data
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TSPLData {
    pub node_coordinates: Option<Vec<Coord>>,
    pub depots: Option<Vec<usize>>,
    pub demands: Option<Vec<u32>>,
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
///Holds a pair or triple of floats.
pub enum Coord {
    Coord2(i64, N64, N64),
    Coord3(i64, N64, N64, N64),
}

///Holds a CVRP Demand for a node
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Demand(pub usize, pub u32);

///`Vec<usize>` of node indices forming a tour
pub type Tour = Vec<usize>;
///`(usize, usize)`
pub type Edge = (usize, usize);
///`Vec<(usize, usize)>`
pub type EdgeList = Vec<Edge>;
///`u32`
pub type EdgeWeight = u32;
///`Vec<u32>`
pub type EdgeWeightList = Vec<EdgeWeight>;
///`Vec<usize>`
pub type Adj = Vec<usize>;

/// Holds edge information, either in the edge list or adjacency list format.
/// The adjacency list version is a vec of `dimension` elements, each of which is a list of
/// connections. Non-connected nodes are still counted as empty lists.
/// TSPLData holds a vec of this type.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EdgeData {
    Edge(Edge),
    Adj(Adj),
}

#[cfg_attr(test, derive(EnumIter))]
#[derive(Debug, PartialEq, Eq, Clone, Display, EnumString)]
/// Specifies the type of the problem.
pub enum ProblemType {
    ///Symmetric Traveling Salesman Problem
    TSP,
    /// Asymmetric Traveling Salesman Problem
    ATSP,
    /// Sequential Ordering Problem
    SOP,
    /// Hamiltonian Cycle Problem
    HCP,
    /// Capacitated Vehicle Routing problem
    CVRP,
    /// A collection of tours
    TOUR,
}

#[cfg_attr(test, derive(EnumIter))]
#[derive(Debug, PartialEq, Eq, Clone, Display, EnumString)]
/// Specifies how the edge weights (or distances) are given.
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

#[cfg_attr(test, derive(EnumIter))]
#[derive(Debug, PartialEq, Eq, Clone, Display, EnumString)]
/// Describes the format of the edge weights if they are given explicitly.
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

#[cfg_attr(test, derive(EnumIter))]
#[derive(Debug, PartialEq, Eq, Clone, Display, EnumString)]
///Describes the format in which the edges of a graph are given, if the graph is not complete.
pub enum EdgeDataFormat {
    EDGE_LIST,
    ADJ_LIST,
}

#[cfg_attr(test, derive(EnumIter))]
#[derive(Debug, PartialEq, Eq, Clone, Display, EnumString)]
///Specifies whether coordinates are associated with each node (which, for example may be used for either graphical display or distance computations).
pub enum NodeCoordType {
    TWOD_COORDS,
    THREED_COORDS,
    NO_COORDS,
}

///Specifies how a graphical display of the nodes can be obtained.
#[cfg_attr(test, derive(EnumIter))]
#[derive(Debug, PartialEq, Eq, Clone, Display, EnumString)]
pub enum DisplayDataType {
    COORDS_DISPLAY,
    TWOD_DISPLAY,
    NO_DISPLAY,
}
