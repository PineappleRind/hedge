// ┌───────────────────────────────────────┐
// │           welcome to Hedge            │
// └───────────────────────────────────────┘

use box_types::{chars_for_type, BoxChars};
use textwrap::Options;

pub mod box_types;
pub mod cli;
pub mod hedge;

fn main() {
    let options = cli::get_args().run();

    let boxed = wrap_in_box(
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris neque sem, dignissim ut augue non, porta porttitor lacus. Vestibulum pharetra elit quis justo tempus dapibus.",
        &options.box_type,
        options.width,
        options.height,
        options.padding
    );
    match boxed {
        Ok(boxed) => println!("{}", boxed),
        Err(e) => println!("Error: {}", e),
    }
}

fn wrap_in_box(
    text: &str,
    box_type: &cli::BoxType,
    width: usize,
    height: usize,
    padding: usize,
) -> Result<String, String> {
    let mut hedged = String::new();

    let chars: BoxChars = chars_for_type(box_type);

    let filled = textwrap::fill(
        text,
        Options::new((width as usize).saturating_sub(padding).saturating_sub(2)),
    );
    let text: Vec<String> = pad_to_width(filled.split('\n').collect::<Vec<&str>>(), width, padding);

    for row in 0..=height {
        let i_want_this_to_be_inlined: String = String::new();
        let current_row = text
            .get((row as usize).saturating_sub(1))
            .unwrap_or(&i_want_this_to_be_inlined);

        for col in 0..=width {
            let str_candidate = hedge::char_for_xy(row, col, width, height, current_row, &chars)?;
            hedged = format!("{}{}", hedged, str_candidate);
        }
        hedged.push('\n');
    }

    Ok(hedged)
}

fn pad_to_width(text: Vec<&str>, width: usize, padding: usize) -> Vec<String> {
    let padded = text.into_iter().map(|row| {
        let spaces_amt = width as i32 - padding as i32 - row.len() as i32;
        println!("{}", spaces_amt);
        let on_each_side: String = " ".repeat(
            (spaces_amt as f64 / 2.0).ceil() as usize + (padding as f64 / 2.0).ceil() as usize,
        );

        format!("{}{}{}", on_each_side, row, on_each_side)
    });

    padded.collect()
}
