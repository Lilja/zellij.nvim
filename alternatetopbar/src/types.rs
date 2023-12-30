use colored::Color;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum PartType {
    Session,
    Spotify,
    Tabs,
    Mode,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Color")]
enum ColorDef {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    TrueColor { r: u8, g: u8, b: u8 },
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Part {
    pub part_type: PartType,
    #[serde(with = "ColorDef")]
    pub bg_color: colored::Color,
    pub number_of_spaces_padding: usize,
    pub use_devicon: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PluginConf {
    pub left: Vec<Part>,
    pub center: Vec<Part>,
    pub right: Vec<Part>,
}
impl Default for PluginConf {
    fn default() -> Self {
        PluginConf {
            left: vec![
                Part {
                    part_type: PartType::Session,
                    bg_color: Color::Red,
                    number_of_spaces_padding: 2,
                    use_devicon: true,
                },
                Part {
                    part_type: PartType::Spotify,
                    bg_color: Color::Magenta,
                    number_of_spaces_padding: 2,
                    use_devicon: true,
                },
            ],
            center: vec![Part {
                part_type: PartType::Tabs,
                bg_color: Color::Yellow,
                number_of_spaces_padding: 2,
                use_devicon: true,
            }],
            right: vec![Part {
                part_type: PartType::Mode,
                bg_color: Color::Green,
                number_of_spaces_padding: 2,
                use_devicon: true,
            }],
        }
    }
}
