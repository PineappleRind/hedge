use textwrap::Options;

use crate::cli::BoxType;
use std::collections::HashMap;

pub struct BoxChars {
    pub corner_nw: String,
    pub corner_ne: String,
    pub corner_sw: String,
    pub corner_se: String,
    pub edge_x: String,
    pub edge_y: String,
}

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

fn pad_to_width(text: Vec<&str>, width: u32) -> Vec<String> {
    let padded = text.into_iter().map(|row| {
        let spaces_amt = width as usize - row.len();

        #[allow(clippy::cast_precision_loss)]
        let on_each_side: usize = (spaces_amt as f64 / 2.0).ceil() as usize;

        let row: String = format!(
            "{}{}{}",
            " ".repeat(on_each_side),
            row,
            " ".repeat(on_each_side)
        );
        row
    });

    padded.collect()
}

// giant function
pub fn char_for_xy(
    box_type: &BoxType,
    row: u32,
    col: u32,
    width: u32,
    height: u32,
    text: &str,
) -> Result<String, String> {
    let BoxChars {
        corner_ne,
        corner_nw,
        corner_sw,
        corner_se,
        edge_x,
        edge_y,
    } = chars_for_type(box_type);

    let filled = textwrap::fill(text, Options::new(width as usize).subsequent_indent(" "));
    let text: Vec<String> = pad_to_width(filled.split('\n').collect(), width);

    let i_want_this_to_be_inlined: String = String::new();
    let current_row = text
        .get((row as usize).saturating_sub(1))
        .unwrap_or(&i_want_this_to_be_inlined);

    let char_candidate = if row == 0 && col == 0 {
        corner_nw
    } else if row == 0 && col == width {
        corner_ne
    } else if row == height && col == 0 {
        corner_sw
    } else if row == height && col == width {
        corner_se
    } else if col == 0 || col == width {
        edge_x
    } else if row == 0 || row == height {
        edge_y
    } else if current_row.as_bytes().get(col as usize).is_some() {
        current_row
            .to_string()
            .chars()
            .nth(col as usize)
            .unwrap_or(' ')
            .to_string()
    } else {
        String::from(" ")
    };

    Ok(char_candidate)
}
