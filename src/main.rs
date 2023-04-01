// ┌───────────────────────────────────────┐
// │           welcome to Hedge            │
// └───────────────────────────────────────┘

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
    );
    match boxed {
        Ok(boxed) => println!("{}", boxed),
        Err(e) => println!("Error: {}", e),
    }
}

fn wrap_in_box(
    text: &str,
    box_type: &cli::BoxType,
    width: u32,
    height: u32,
) -> Result<String, String> {
    let mut hedged = String::new();
    for row in 0..=height {
        for col in 0..=width {
            let str_candidate = hedge::char_for_xy(box_type, row, col, width, height, text)?;
            hedged = format!("{}{}", hedged, str_candidate);
        }
        hedged.push('\n');
    }

    Ok(hedged)
}
