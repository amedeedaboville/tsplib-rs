use noisy_float::prelude::*;
use pretty_assertions::assert_eq;
use std::fs;
use tsplib::Coord::Coord2;
use tsplib::*;

#[test]
fn berlin52() {
    let filename = "tests/testdata/berlin52.tsp";
    let file_contents = fs::read_to_string(filename).unwrap();
    let parsed = parse_problem(&file_contents).unwrap().1;
    let mut t = TSPLData::empty();
    let header = TSPLMeta {
        comment: "52 locations in Berlin (Groetschel)".to_string(),
        dimension: 52,
        display_data_type: DisplayDataType::NO_DISPLAY,
        name: "berlin52".to_string(),
        problem_type: ProblemType::TSP,
        capacity: None,
        edge_weight_type: EdgeWeightType::EUC_2D,
        edge_weight_format: None,
        edge_data_format: None,
        node_coord_type: NodeCoordType::NO_COORDS,
    };
    t.node_coordinates = Some(vec![
        Coord2(1, n64(565.0), n64(575.0)),
        Coord2(2, n64(25.0), n64(185.0)),
        Coord2(3, n64(345.0), n64(750.0)),
        Coord2(4, n64(945.0), n64(685.0)),
        Coord2(5, n64(845.0), n64(655.0)),
        Coord2(6, n64(880.0), n64(660.0)),
        Coord2(7, n64(25.0), n64(230.0)),
        Coord2(8, n64(525.0), n64(1000.0)),
        Coord2(9, n64(580.0), n64(1175.0)),
        Coord2(10, n64(650.0), n64(1130.0)),
        Coord2(11, n64(1605.0), n64(620.0)),
        Coord2(12, n64(1220.0), n64(580.0)),
        Coord2(13, n64(1465.0), n64(200.0)),
        Coord2(14, n64(1530.0), n64(5.0)),
        Coord2(15, n64(845.0), n64(680.0)),
        Coord2(16, n64(725.0), n64(370.0)),
        Coord2(17, n64(145.0), n64(665.0)),
        Coord2(18, n64(415.0), n64(635.0)),
        Coord2(19, n64(510.0), n64(875.0)),
        Coord2(20, n64(560.0), n64(365.0)),
        Coord2(21, n64(300.0), n64(465.0)),
        Coord2(22, n64(520.0), n64(585.0)),
        Coord2(23, n64(480.0), n64(415.0)),
        Coord2(24, n64(835.0), n64(625.0)),
        Coord2(25, n64(975.0), n64(580.0)),
        Coord2(26, n64(1215.0), n64(245.0)),
        Coord2(27, n64(1320.0), n64(315.0)),
        Coord2(28, n64(1250.0), n64(400.0)),
        Coord2(29, n64(660.0), n64(180.0)),
        Coord2(30, n64(410.0), n64(250.0)),
        Coord2(31, n64(420.0), n64(555.0)),
        Coord2(32, n64(575.0), n64(665.0)),
        Coord2(33, n64(1150.0), n64(1160.0)),
        Coord2(34, n64(700.0), n64(580.0)),
        Coord2(35, n64(685.0), n64(595.0)),
        Coord2(36, n64(685.0), n64(610.0)),
        Coord2(37, n64(770.0), n64(610.0)),
        Coord2(38, n64(795.0), n64(645.0)),
        Coord2(39, n64(720.0), n64(635.0)),
        Coord2(40, n64(760.0), n64(650.0)),
        Coord2(41, n64(475.0), n64(960.0)),
        Coord2(42, n64(95.0), n64(260.0)),
        Coord2(43, n64(875.0), n64(920.0)),
        Coord2(44, n64(700.0), n64(500.0)),
        Coord2(45, n64(555.0), n64(815.0)),
        Coord2(46, n64(830.0), n64(485.0)),
        Coord2(47, n64(1170.0), n64(65.0)),
        Coord2(48, n64(830.0), n64(610.0)),
        Coord2(49, n64(605.0), n64(625.0)),
        Coord2(50, n64(595.0), n64(360.0)),
        Coord2(51, n64(1340.0), n64(725.0)),
        Coord2(52, n64(1740.0), n64(245.0)),
    ]);
    assert_eq!(
        parsed,
        TSPLProblem {
            header: header,
            data: t,
        }
    );
}

#[test]
fn gr17() {
    let filename = "tests/testdata/gr17.tsp";
    let file_contents = fs::read_to_string(filename).unwrap();
    let parsed = parse_problem(&file_contents).unwrap().1;
    let mut t = TSPLData::empty();
    let header = TSPLMeta {
        comment: "17-city problem (Groetschel)".to_string(),
        dimension: 17,
        display_data_type: DisplayDataType::NO_DISPLAY,
        name: "gr17".to_string(),
        problem_type: ProblemType::TSP,
        capacity: None,
        edge_weight_type: EdgeWeightType::EXPLICIT,
        edge_data_format: None,
        edge_weight_format: Some(EdgeWeightFormat::LOWER_DIAG_ROW),
        node_coord_type: NodeCoordType::NO_COORDS,
    };
    t.edge_weights = Some(vec![
        0, 633, 0, 257, 390, 0, 91, 661, 228, 0, 412, 227, 169, 383, 0, 150, 488, 112, 120, 267, 0,
        80, 572, 196, 77, 351, 63, 0, 134, 530, 154, 105, 309, 34, 29, 0, 259, 555, 372, 175, 338,
        264, 232, 249, 0, 505, 289, 262, 476, 196, 360, 444, 402, 495, 0, 353, 282, 110, 324, 61,
        208, 292, 250, 352, 154, 0, 324, 638, 437, 240, 421, 329, 297, 314, 95, 578, 435, 0, 70,
        567, 191, 27, 346, 83, 47, 68, 189, 439, 287, 254, 0, 211, 466, 74, 182, 243, 105, 150,
        108, 326, 336, 184, 391, 145, 0, 268, 420, 53, 239, 199, 123, 207, 165, 383, 240, 140, 448,
        202, 57, 0, 246, 745, 472, 237, 528, 364, 332, 349, 202, 685, 542, 157, 289, 426, 483, 0,
        121, 518, 142, 84, 297, 35, 29, 36, 236, 390, 238, 301, 55, 96, 153, 336, 0,
    ]);
    assert_eq!(
        parsed,
        TSPLProblem {
            header: header,
            data: t,
        }
    );
}

#[test]
fn bays29() {
    let filename = "tests/testdata/bays29.tsp";
    let file_contents = fs::read_to_string(filename).unwrap();
    let parsed = parse_problem(&file_contents).unwrap().1;
    let mut t = TSPLData::empty();
    let header = TSPLMeta {
        comment: "29 cities in Bavaria, street distances (Groetschel,Juenger,Reinelt)".to_string(),
        dimension: 29,
        display_data_type: DisplayDataType::TWOD_DISPLAY,
        name: "bays29".to_string(),
        problem_type: ProblemType::TSP,
        capacity: None,
        edge_weight_type: EdgeWeightType::EXPLICIT,
        edge_data_format: None,
        edge_weight_format: Some(EdgeWeightFormat::FULL_MATRIX),
        node_coord_type: NodeCoordType::NO_COORDS,
    };
    t.edge_weights = Some(vec![
        0, 107, 241, 190, 124, 80, 316, 76, 152, 157, 283, 133, 113, 297, 228, 129, 348, 276, 188,
        150, 65, 341, 184, 67, 221, 169, 108, 45, 167, 107, 0, 148, 137, 88, 127, 336, 183, 134,
        95, 254, 180, 101, 234, 175, 176, 265, 199, 182, 67, 42, 278, 271, 146, 251, 105, 191, 139,
        79, 241, 148, 0, 374, 171, 259, 509, 317, 217, 232, 491, 312, 280, 391, 412, 349, 422, 356,
        355, 204, 182, 435, 417, 292, 424, 116, 337, 273, 77, 190, 137, 374, 0, 202, 234, 222, 192,
        248, 42, 117, 287, 79, 107, 38, 121, 152, 86, 68, 70, 137, 151, 239, 135, 137, 242, 165,
        228, 205, 124, 88, 171, 202, 0, 61, 392, 202, 46, 160, 319, 112, 163, 322, 240, 232, 314,
        287, 238, 155, 65, 366, 300, 175, 307, 57, 220, 121, 97, 80, 127, 259, 234, 61, 0, 386,
        141, 72, 167, 351, 55, 157, 331, 272, 226, 362, 296, 232, 164, 85, 375, 249, 147, 301, 118,
        188, 60, 185, 316, 336, 509, 222, 392, 386, 0, 233, 438, 254, 202, 439, 235, 254, 210, 187,
        313, 266, 154, 282, 321, 298, 168, 249, 95, 437, 190, 314, 435, 76, 183, 317, 192, 202,
        141, 233, 0, 213, 188, 272, 193, 131, 302, 233, 98, 344, 289, 177, 216, 141, 346, 108, 57,
        190, 245, 43, 81, 243, 152, 134, 217, 248, 46, 72, 438, 213, 0, 206, 365, 89, 209, 368,
        286, 278, 360, 333, 284, 201, 111, 412, 321, 221, 353, 72, 266, 132, 111, 157, 95, 232, 42,
        160, 167, 254, 188, 206, 0, 159, 220, 57, 149, 80, 132, 193, 127, 100, 28, 95, 193, 241,
        131, 169, 200, 161, 189, 163, 283, 254, 491, 117, 319, 351, 202, 272, 365, 159, 0, 404,
        176, 106, 79, 161, 165, 141, 95, 187, 254, 103, 279, 215, 117, 359, 216, 308, 322, 133,
        180, 312, 287, 112, 55, 439, 193, 89, 220, 404, 0, 210, 384, 325, 279, 415, 349, 285, 217,
        138, 428, 310, 200, 354, 169, 241, 112, 238, 113, 101, 280, 79, 163, 157, 235, 131, 209,
        57, 176, 210, 0, 186, 117, 75, 231, 165, 81, 85, 92, 230, 184, 74, 150, 208, 104, 158, 206,
        297, 234, 391, 107, 322, 331, 254, 302, 368, 149, 106, 384, 186, 0, 69, 191, 59, 35, 125,
        167, 255, 44, 309, 245, 169, 327, 246, 335, 288, 228, 175, 412, 38, 240, 272, 210, 233,
        286, 80, 79, 325, 117, 69, 0, 122, 122, 56, 56, 108, 175, 113, 240, 176, 125, 280, 177,
        266, 243, 129, 176, 349, 121, 232, 226, 187, 98, 278, 132, 161, 279, 75, 191, 122, 0, 244,
        178, 66, 160, 161, 235, 118, 62, 92, 277, 55, 155, 275, 348, 265, 422, 152, 314, 362, 313,
        344, 360, 193, 165, 415, 231, 59, 122, 244, 0, 66, 178, 198, 286, 77, 362, 287, 228, 358,
        299, 380, 319, 276, 199, 356, 86, 287, 296, 266, 289, 333, 127, 141, 349, 165, 35, 56, 178,
        66, 0, 112, 132, 220, 79, 296, 232, 181, 292, 233, 314, 253, 188, 182, 355, 68, 238, 232,
        154, 177, 284, 100, 95, 285, 81, 125, 56, 66, 178, 112, 0, 128, 167, 169, 179, 120, 69,
        283, 121, 213, 281, 150, 67, 204, 70, 155, 164, 282, 216, 201, 28, 187, 217, 85, 167, 108,
        160, 198, 132, 128, 0, 88, 211, 269, 159, 197, 172, 189, 182, 135, 65, 42, 182, 137, 65,
        85, 321, 141, 111, 95, 254, 138, 92, 255, 175, 161, 286, 220, 167, 88, 0, 299, 229, 104,
        236, 110, 149, 97, 108, 341, 278, 435, 151, 366, 375, 298, 346, 412, 193, 103, 428, 230,
        44, 113, 235, 77, 79, 169, 211, 299, 0, 353, 289, 213, 371, 290, 379, 332, 184, 271, 417,
        239, 300, 249, 168, 108, 321, 241, 279, 310, 184, 309, 240, 118, 362, 296, 179, 269, 229,
        353, 0, 121, 162, 345, 80, 189, 342, 67, 146, 292, 135, 175, 147, 249, 57, 221, 131, 215,
        200, 74, 245, 176, 62, 287, 232, 120, 159, 104, 289, 121, 0, 154, 220, 41, 93, 218, 221,
        251, 424, 137, 307, 301, 95, 190, 353, 169, 117, 354, 150, 169, 125, 92, 228, 181, 69, 197,
        236, 213, 162, 154, 0, 352, 147, 247, 350, 169, 105, 116, 242, 57, 118, 437, 245, 72, 200,
        359, 169, 208, 327, 280, 277, 358, 292, 283, 172, 110, 371, 345, 220, 352, 0, 265, 178, 39,
        108, 191, 337, 165, 220, 188, 190, 43, 266, 161, 216, 241, 104, 246, 177, 55, 299, 233,
        121, 189, 149, 290, 80, 41, 147, 265, 0, 124, 263, 45, 139, 273, 228, 121, 60, 314, 81,
        132, 189, 308, 112, 158, 335, 266, 155, 380, 314, 213, 182, 97, 379, 189, 93, 247, 178,
        124, 0, 199, 167, 79, 77, 205, 97, 185, 435, 243, 111, 163, 322, 238, 206, 288, 243, 275,
        319, 253, 281, 135, 108, 332, 342, 218, 350, 39, 263, 199, 0,
    ]);
    t.display_data = Some(vec![
        Coord2(1, n64(1150.0), n64(1760.0)),
        Coord2(2, n64(630.0), n64(1660.0)),
        Coord2(3, n64(40.0), n64(2090.0)),
        Coord2(4, n64(750.0), n64(1100.0)),
        Coord2(5, n64(750.0), n64(2030.0)),
        Coord2(6, n64(1030.0), n64(2070.0)),
        Coord2(7, n64(1650.0), n64(650.0)),
        Coord2(8, n64(1490.0), n64(1630.0)),
        Coord2(9, n64(790.0), n64(2260.0)),
        Coord2(10, n64(710.0), n64(1310.0)),
        Coord2(11, n64(840.0), n64(550.0)),
        Coord2(12, n64(1170.0), n64(2300.0)),
        Coord2(13, n64(970.0), n64(1340.0)),
        Coord2(14, n64(510.0), n64(700.0)),
        Coord2(15, n64(750.0), n64(900.0)),
        Coord2(16, n64(1280.0), n64(1200.0)),
        Coord2(17, n64(230.0), n64(590.0)),
        Coord2(18, n64(460.0), n64(860.0)),
        Coord2(19, n64(1040.0), n64(950.0)),
        Coord2(20, n64(590.0), n64(1390.0)),
        Coord2(21, n64(830.0), n64(1770.0)),
        Coord2(22, n64(490.0), n64(500.0)),
        Coord2(23, n64(1840.0), n64(1240.0)),
        Coord2(24, n64(1260.0), n64(1500.0)),
        Coord2(25, n64(1280.0), n64(790.0)),
        Coord2(26, n64(490.0), n64(2130.0)),
        Coord2(27, n64(1460.0), n64(1420.0)),
        Coord2(28, n64(1260.0), n64(1910.0)),
        Coord2(29, n64(360.0), n64(1980.0)),
    ]);
    assert_eq!(
        parsed,
        TSPLProblem {
            header: header,
            data: t,
        }
    );
}

#[test]
fn parse_alltsp() {
    let paths = fs::read_dir("examples/alltsp/problems").unwrap();

    for path in paths {
        let pathpath = path.unwrap().path();
        let pathstr = pathpath.to_str().unwrap();
        println!("path is: {}", pathstr);
        if !pathstr.ends_with(".tsp") {
            continue;
        };
        let contents = fs::read_to_string(pathstr).unwrap();
        let parsed = parse_problem(&contents);
        // println!("parsed got {:?}", parsed);
        match tsplib::parse_file(&pathstr) {
            Some(_) => continue,
            None => {println!("Error parsing this one, got {:?}", parsed) }
        };
    }
}

#[test]
fn parse_allatsp() {
    let paths = fs::read_dir("examples/allatsp").unwrap();

    for path in paths {
        let pathpath = path.unwrap().path();
        let pathstr = pathpath.to_str().unwrap();
        if !pathstr.ends_with(".atsp") {
            continue;
        };
        println!("path is: {}", pathstr);
        let contents = fs::read_to_string(pathstr).unwrap();
        let parsed = parse_problem(&contents);
        // println!("parsed got {:?}", parsed);
        match tsplib::parse_file(&pathstr) {
            Some(_) => continue,
            None => {println!("Error parsing this one, got {:?}", parsed) }
        };
    }
}