use std::collections::HashMap;

use crate::cli::BoxType;

pub struct BoxChars {
    pub corner_nw: char,
    pub corner_ne: char,
    pub corner_sw: char,
    pub corner_se: char,
    pub edge_x: char,
    pub edge_y: char,
}

pub fn chars_for_type(box_type: &BoxType) -> BoxChars {
    let mut chars: HashMap<&str, BoxChars> = HashMap::from([
        (
            "solid",
            BoxChars {
                corner_nw: '┌',
                corner_ne: '┐',
                corner_sw: '└',
                corner_se: '┘',
                edge_x: '│',
                edge_y: '─',
            },
        ),
        (
            "solid_round",
            BoxChars {
                corner_nw: '╭',
                corner_ne: '╮',
                corner_sw: '╰',
                corner_se: '╯',
                edge_x: '│',
                edge_y: '─',
            },
        ),
    ]);

    let box_type = match box_type {
        BoxType::Solid => "solid",
        BoxType::SolidRound => "solid_round",
    };

    chars.remove(box_type).unwrap()
}
