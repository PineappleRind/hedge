use std::str::FromStr;

use bpaf::{construct, short, OptionParser, Parser};

#[allow(dead_code)]
#[derive(Debug)]
pub struct HedgeOptions {
    pub box_type: BoxType,
    pub width: u32,
    pub height: u32,
    // pub round: bool,
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
        .help("Type of box")
        .argument::<BoxType>("TYPE");

    let width = short('w')
        .long("width")
        .help("Width of box (defaults to auto)")
        .argument::<u32>("INT");

    let height = short('h')
        .long("height")
        .help("Height of box (defaults to auto)")
        .argument::<u32>("INT");

    // let round = short('r')
    //     .long("round")
    //     .help("round corners of the box")
    //     .switch()
    //     .fallback(false);

    construct!(HedgeOptions {
        box_type,
        width,
        height,
        // round
    })
    .to_options()
    .descr("Wrap text in ASCII boxes")
}
