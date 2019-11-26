//use crate::enums::*;

// fn build_distance_matrix(
//     dimension: usize,
//     ewf: &EdgeWeightFormat,
//     edges: Vec<Vec<u32>>,
// ) -> Option<EdgeWeightMatrix> {
//     if *ewf == EdgeWeightFormat::FUNCTION {
//         return None;
//     }
//     let mut res: EdgeWeightMatrix = vec![vec![std::u32::MAX; dimension]; dimension];
//     let mut r: usize = 0;
//     let mut c: usize = 0;

//     let rc_range: Vec<(usize)> = match ewf {
//         EdgeWeightFormat::FULL_MATRIX => (0..dimension).flat_map(|_r| 0..dimension).collect(),
//         EdgeWeightFormat::LOWER_ROW => (1..dimension).flat_map(|r| 0..(r - 1)).collect(),
//         EdgeWeightFormat::UPPER_COL => (1..dimension).flat_map(|r| 0..(r - 1)).collect(),
//         EdgeWeightFormat::LOWER_DIAG_ROW => (0..dimension).flat_map(|r| 0..r).collect(),
//         EdgeWeightFormat::UPPER_DIAG_COL => (0..dimension).flat_map(|r| 0..r).collect(),
//         EdgeWeightFormat::UPPER_DIAG_ROW => (0..dimension).flat_map(|r| r..dimension).collect(),
//         EdgeWeightFormat::LOWER_DIAG_COL => (0..dimension).flat_map(|r| r..dimension).collect(),
//         EdgeWeightFormat::UPPER_ROW => (0..(dimension - 1))
//             .flat_map(|r| (r + 1)..dimension)
//             .collect(),
//         EdgeWeightFormat::LOWER_COL => (0..(dimension - 1))
//             .flat_map(|r| (r + 1)..dimension)
//             .collect(),
//     };

//     for weight in edges.flatten() {
//         (r, c) = rc_range.next();
//         res[r][c] = weight;
//     }
// }

// //The TSPLIB concrete edge weights are given as lists of numbers.
// //This is the easiest one, but we need to collect all the vectors together first
// fn build_full_matrix(
//     dimension: usize,
//     ewf: &EdgeWeightFormat,
//     edges: Vec<Vec<u32>>,
// ) -> Option<EdgeWeightMatrix> {
//     if *ewf == EdgeWeightFormat::FUNCTION {
//         return None;
//     }
//     let mut combined: Vec<u32> = Vec::<u32>::new();
//     for line in edges.into_iter() {
//         combined.extend(line);
//     }

//     match ewf {
//         EdgeWeightFormat::FULL_MATRIX => Some(combined.chunks(dimension).collect()),
//         _ => None,
//     }
// }
