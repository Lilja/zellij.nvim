use colored::{Color, ColoredString, Colorize};

const RIGHT_SEP: &str = "\u{E0B0}";
const LEFT_SEP: &str = "\u{E0B2}";

#[derive(PartialEq, Eq)]
pub enum SeperatorDirection {
    Right,
    Left,
}
pub struct DirectionOption {
    pub direction: SeperatorDirection,
    pub on: Option<Color>,
    pub color: Option<Color>,
}
pub struct ColorizeTokenOptions {
    pub start: Option<DirectionOption>,
    pub end: Option<DirectionOption>,
    pub text: String,
    pub color: Option<Color>,
    pub on: Option<Color>,
    pub pad_string: bool,
}
pub fn create_seperated_formated_text(opts: ColorizeTokenOptions) -> String {
    let output = color_token(&opts.text, opts.color, opts.on, opts.pad_string);
    let mut _start = String::new();
    let mut _end = String::new();
    match opts.start {
        Some(start) => {
            _start = color_token(
                if start.direction == SeperatorDirection::Left {
                    LEFT_SEP
                } else {
                    RIGHT_SEP
                },
                start.color,
                start.on,
                false,
            )
        }
        None => {}
    }
    match opts.end {
        Some(end) => {
            let sep = if end.direction == SeperatorDirection::Left {
                LEFT_SEP
            } else {
                RIGHT_SEP
            };
            _end = color_token(sep, end.color, end.on, false);
        }
        None => {}
    }
    return format!("{}{}{}", _start, output, _end);
}

fn color_token(text: &str, fg: Option<Color>, bg: Option<Color>, pad_string: bool) -> String {
    let padded_output = if pad_string {
        format!(" {} ", text)
    } else {
        format!("{}", text)
    };

    let mut xd: Option<ColoredString> = None;
    if fg.is_some() {
        xd = Some(padded_output.color(fg.unwrap()))
    }
    if bg.is_some() {
        if xd.is_some() {
            xd = Some(xd.unwrap().on_color(bg.unwrap()))
        } else {
            xd = Some(padded_output.on_color(bg.unwrap()))
        }
    }
    return match xd {
        Some(a) => format!("{}", a),
        None => String::new(),
    };
}

pub fn cap_tab_text(text: String, extra_space: usize, free_space_per_tab: usize) -> String {
    // Do we need to cut the tab name?
    let total_space = text.len() + extra_space;
    // eprintln!("Is too large? text only: {} text+whitespcae: {} free: {}", text.len(), total_space, free_space_per_tab);
    if total_space > free_space_per_tab {
        // eprintln!("Capping, size: {}, free space: {}, deriv: {}", total_space, free_space_per_tab, free_space_per_tab-4);
        return format!("{}...", text[..free_space_per_tab - 3 - extra_space].to_string());
    }
    return text;
}
