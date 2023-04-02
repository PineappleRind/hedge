use std::str::FromStr;

use bpaf::{construct, short, OptionParser, Parser};

#[allow(dead_code)]
#[derive(Debug)]
pub struct HedgeOptions {
    pub box_type: BoxType,
    pub width: usize,
    pub height: usize,
    pub padding: usize,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum BoxType {
    Solid,
    SolidRound,
}

impl FromStr for BoxType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        match s {
            "solid" => Ok(BoxType::Solid),
            "solid_round" => Ok(BoxType::SolidRound),
            _ => Err("Expected solid".to_string()),
        }
    }
}

pub fn get_args() -> OptionParser<HedgeOptions> {
    let box_type = short('t')
        .long("type")
        .help("Type of box (one of solid, solid_round)")
        .argument::<BoxType>("TYPE");

    let width = short('w')
        .long("width")
        .help("Width of box, defaults to 0 (auto)")
        .argument::<usize>("INT");

    let height = short('h')
        .long("height")
        .help("Height of box, defaults to 0 (auto)")
        .argument::<usize>("INT");

    let padding = short('p')
        .long("padding")
        .help("X padding (defaults to 2)")
        .argument::<usize>("INT")
        .fallback(2);

    construct!(HedgeOptions {
        box_type,
        width,
        height,
        padding,
    })
    .to_options()
    .descr("Wrap text in ASCII boxes")
}
