use std::collections::HashMap;

use crate::cli::BoxType;

pub struct BoxChars {
    pub corner_nw: String,
    pub corner_ne: String,
    pub corner_sw: String,
    pub corner_se: String,
    pub edge_x: String,
    pub edge_y: String,
}

pub fn chars_for_type(box_type: &BoxType) -> BoxChars {
    let mut chars: HashMap<&str, BoxChars> = HashMap::from([
        (
            "solid",
            BoxChars {
                corner_nw: "┌".to_string(),
                corner_ne: "┐".to_string(),
                corner_sw: "└".to_string(),
                corner_se: "┘".to_string(),
                edge_x: "│".to_string(),
                edge_y: "─".to_string(),
            },
        ),
        (
            "solid_round",
            BoxChars {
                corner_nw: "╭".to_string(),
                corner_ne: "╮".to_string(),
                corner_sw: "╰".to_string(),
                corner_se: "╯".to_string(),
                edge_x: "│".to_string(),
                edge_y: "─".to_string(),
            },
        ),
    ]);

    let box_type = match box_type {
        BoxType::Solid => "solid".to_string(),
        BoxType::SolidRound => "solid_round".to_string(),
    };

    chars.remove(box_type.as_str()).unwrap()
}
