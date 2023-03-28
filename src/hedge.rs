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

struct BoundingBox(u32, u32, u32, u32);

fn chars_for_type(box_type: &BoxType) -> BoxChars {
    let mut chars: HashMap<&str, BoxChars> = HashMap::from([(
        "solid",
        BoxChars {
            corner_nw: "┌".to_string(),
            corner_ne: "┐".to_string(),
            corner_sw: "└".to_string(),
            corner_se: "┘".to_string(),
            edge_x: "│".to_string(),
            edge_y: "─".to_string(),
        },
    )]);

    let box_type = match box_type {
        BoxType::Solid => "solid".to_string(),
    };

    chars.remove(box_type.as_str()).unwrap()
}

fn _text_bounding_box(_text: &str, _width: u32, _height: u32) -> BoundingBox {
    BoundingBox(0, 0, 0, 0)
}

pub fn char_for_xy(
    box_type: &BoxType,
    row: u32,
    col: u32,
    width: u32,
    height: u32,
) -> Result<String, String> {
    let BoxChars {
        corner_ne,
        corner_nw,
        corner_sw,
        corner_se,
        edge_x,
        edge_y,
    } = chars_for_type(box_type);

    // must be a better way to do this...
    let char_candidate = if row == 0 && col == 0 {
        corner_nw
    } else if row == width && col == 0 {
        corner_ne
    } else if row == 0 && col == height {
        corner_sw
    } else if row == width && col == height {
        corner_se
    } else if row == 0 || row == width {
        edge_x
    } else if col == 0 || col == height {
        edge_y
    } else {
        String::new()
    };
    Ok(char_candidate)
}
