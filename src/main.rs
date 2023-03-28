pub mod chars;
pub mod cli;
/*
Hedge v0
Takes text and wraps in a box.
Like this */
fn main() {
    let options = cli::get_args().run();

    let boxed = wrap_in_box(&"wahoo", options.box_type, options.width, options.height);
    println!("{}", boxed);
}

fn wrap_in_box(_text: &str, box_type: cli::BoxType, width: u32, height: u32) -> String {
    let chars::BoxChars {
        corner_ne,
        corner_nw,
        corner_sw,
        corner_se,
        edge_x,
        edge_y,
    } = chars::chars_for_type(box_type).unwrap();

    let mut hedged = "".to_string();

    for col in 0..=height {
        for row in 0..=width {
            let str_candidate = if row == 0 && col == 0 {
                corner_nw.to_owned()
            } else if row == width && col == 0 {
                corner_ne.to_owned()
            } else if row == 0 && col == height {
                corner_sw.to_owned()
            } else if row == width && col == height {
                corner_se.to_owned()
            } else if row == 0 || row == width {
                edge_x.to_owned()
            } else if col == 0 || col == height {
                edge_y.to_owned()
            } else {
                " ".to_string()
            };
            hedged = format!("{}{}", hedged, str_candidate);
        }
        hedged.push_str("\n");
    }

    hedged
}
