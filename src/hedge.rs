use crate::box_types::BoxChars;



// giant function
pub fn char_for_xy(
    row: usize,
    col: usize,
    width: usize,
    height: usize,
    current_row: &str,
    chars: &BoxChars
) -> Result<char, String> {
    let char_candidate = if row == 0 && col == 0 {
        chars.corner_nw
    } else if row == 0 && col == width {
        chars.corner_ne
    } else if row == height && col == 0 {
        chars.corner_sw
    } else if row == height && col == width {
        chars.corner_se
    } else if col == 0 || col == width {
        chars.edge_x
    } else if row == 0 || row == height {
        chars.edge_y
    } else if current_row.as_bytes().get(col as usize).is_some() {
        current_row
            .chars()
            .nth(col as usize)
            .unwrap_or(' ')
    } else {
        ' '
    };

    Ok(char_candidate)
}
