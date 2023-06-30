use std::cmp::min;
use std::io::{self, Write};

use colored::{Color, ColoredString, Colorize};
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;
use zellij_tile::prelude::*;

#[derive(Default)]
struct State {
    tabs: Vec<TabInfo>,
    active_tab_idx: usize,
    mode_info: ModeInfo,
    //mouse_click_pos: usize,
}

register_plugin!(State);

const RIGHT_SEP: &str = "\u{E0B0}";
const LEFT_SEP: &str = "\u{E0B2}";

impl ZellijPlugin for State {
    fn load(&mut self) {
        set_selectable(false);
        subscribe(&[EventType::TabUpdate, EventType::ModeUpdate])
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::ModeUpdate(mode_info) => {
                self.mode_info = mode_info;
                true
            }
            Event::TabUpdate(tabs) => {
                self.active_tab_idx = tabs.iter().position(|t| t.active).unwrap() + 1;
                let mut new_tabs: Vec<TabInfo> = vec![];
                for (i, tab) in tabs.iter().enumerate() {
                    let temp = TabInfo {
                        active_swap_layout_name: tab.active_swap_layout_name.clone(),
                        is_swap_layout_dirty: tab.is_swap_layout_dirty,
                        position: tab.position,
                        name: tab.name.to_owned(),
                        active: tab.active,
                        panes_to_hide: tab.panes_to_hide,
                        is_fullscreen_active: tab.is_fullscreen_active,
                        is_sync_panes_active: tab.is_sync_panes_active,
                        are_floating_panes_visible: tab.are_floating_panes_visible,
                        other_focused_clients: tab.clone().other_focused_clients,
                    };
                    new_tabs.push(temp);
                }
                self.tabs = new_tabs;
                true
            }
            _ => false,
        }
    }

    fn render(&mut self, rows: usize, cols: usize) {
        let mode = self.mode_info.mode;
        let session_name = &self.mode_info.session_name;
        /*
         * <-----------------------------> col     - e.g. 160
         * <--------------|--------------> middle = col / 2
         * <text>                          s.len() - e.g. 60
         * <text><-----------------------> padding needed = col - s.len()
         *       <------------>            padding needed left side = padding needed/2
         *
         * <-----------><text><---------->
         */

        /*
         * <-----------------------------> col     - e.g. 160
         *             <Normal>            Mode part (<No>)
         *          <Tab 1, Tab 2>         Tab display (<Ta>)
         *             <Session>           Zellij session name (<Se>)
         * <--------------|--------------> middle = col / 2
         * <Se><-------><Ta>               left padding = middle - len(<Left>) - len(<Middle>)
         *              <Ta><--------><Se> right padding = middle - len(<Righ>) - len(<Middle>)
         *
         * */

        let left = create_session_name(session_name.clone());
        let mode = create_mode_part(&mode);
        let free_space_without_center = cols - mode.number_of_chars - left.number_of_chars;
        let tabs = create_tabs(&self.tabs, free_space_without_center);
        let total_unused_space = cols - tabs.number_of_chars;
        if tabs.number_of_chars > cols {
            println!("Out of bounds");
            return;
        }
        let middle = find_middle(cols);

        let free_space = if tabs.number_of_chars > free_space_without_center {
            0
        } else {
            free_space_without_center - tabs.number_of_chars
        };
        let tab_size = fix_wonky_programming(tabs.number_of_chars);
        let no_left_padding =
            calculate_left_padding(middle, left.number_of_chars, tabs.number_of_chars);
        let left_padding = " ".repeat(no_left_padding).on_black();
        let no_right_padding =
            calculate_right_padding(middle, mode.number_of_chars, tab_size, no_left_padding, free_space);
        let right_padding = " ".repeat(no_right_padding).on_black();
        println!(
            "Cols: {}, Middle: {}, Left size: {}, Tab size: {}, Right size: {}, left_padding: {left_padding}, right_padding: {right_padding}, free_space: {free_space}",
            cols,
            middle,
            left.number_of_chars,
            tabs.number_of_chars,
            mode.number_of_chars,
            left_padding = no_left_padding,
            right_padding = no_right_padding,
            free_space = free_space,
        );
        /*
        println!(
            "Left padding {}, Right padding: {}",
            (total_unused_space - left.len()) / 2,
            (total_unused_space - (zellij_human_readable_mode.len() + 3)) / 2,
        );
        */

        println!(
            "{left}{left_padding}{center}{right_padding}{right}",
            left_padding = left_padding,
            right_padding = right_padding,
            center = tabs.text,
            left = left.text,
            right = mode.text,
        );
        println!("{left}{m}{right}", left = "*".repeat(middle - 1), m = "I".yellow(), right="*".repeat(middle-1));
        println!(
            "left   size: {}, padding: {}",
            left.number_of_chars, no_left_padding
        );
        println!("center size: {}, divided and rounded: {}. Should be +4 because of pad+seperators", tabs.number_of_chars, tab_size);
        println!(
            "right  size: {}, padding: {}",
            mode.number_of_chars, no_right_padding
        );
        println!("cols: {}, middle: {}", cols, middle);
    }
}

struct ContentLol {
    number_of_chars: usize,
    text: String,
}

fn create_tabs(tabs: &Vec<TabInfo>, free_space: usize) -> ContentLol {
    let mut s = String::new();
    let mut chars: usize = 0;

    let number_of_tabs = tabs.len();
    tabs.iter().for_each(|tab| {
        let name = &tab.name;
        // 2: 1 space before text, 1 after.
        // 2: Seperator before and after
        let extra_space_per_tab = 2 + 2;
        let capped_text = cap_tab_text(
            name.to_string(),
            extra_space_per_tab,
            free_space / number_of_tabs,
        );
        let capped_text_size = capped_text.len();
        eprintln!("New text size: {}", capped_text_size);
        if tab.active {
            s.push_str(&create_seperated_formated_text(ColorizeTokenOptions {
                start: Some(DirectionOption {
                    on: Some(Color::Yellow),
                    color: Some(Color::Black),
                    direction: SeperatorDirection::Right,
                }),
                end: Some(DirectionOption {
                    on: Some(Color::Black),
                    color: Some(Color::Yellow),
                    direction: SeperatorDirection::Right,
                }),
                text: capped_text,
                on: Some(Color::Yellow),
                color: Some(Color::Black),
                pad_string: true,
            }));
        } else {
            s.push_str(&create_seperated_formated_text(ColorizeTokenOptions {
                start: Some(DirectionOption {
                    on: Some(Color::BrightBlack),
                    color: Some(Color::Black),
                    direction: SeperatorDirection::Right,
                }),
                end: Some(DirectionOption {
                    on: Some(Color::Black),
                    color: Some(Color::BrightBlack),
                    direction: SeperatorDirection::Right,
                }),
                text: capped_text,
                on: Some(Color::BrightBlack),
                color: None,
                pad_string: true,
            }));
        }
        // Space left and right of tab name
        chars = chars + 2;
        // Tab name itself
        chars = chars + capped_text_size;
        // Left and right separator
        chars = chars + 2;
    });

    let number_of_spaces_between_tabs: usize = tabs.len() - 1;

    return ContentLol {
        number_of_chars: chars + number_of_spaces_between_tabs,
        text: s,
    };
}

fn create_mode_part(mode: &InputMode) -> ContentLol {
    let zellij_human_readable_mode = format!("{:?}", mode).to_uppercase();

    let size = zellij_human_readable_mode.len();
    let text = create_seperated_formated_text(ColorizeTokenOptions {
        start: Some(DirectionOption {
            direction: SeperatorDirection::Left,
            on: Some(Color::Black),
            color: Some(Color::Green),
        }),
        end: None,
        text: zellij_human_readable_mode,
        color: Some(Color::Black),
        on: Some(Color::Green),
        pad_string: true,
    });

    return ContentLol {
        // 2 spaces + 1 seperator
        number_of_chars: size + 2 + 1,
        text,
    };
}

#[derive(PartialEq, Eq)]
enum SeperatorDirection {
    Right,
    Left,
}
struct DirectionOption {
    direction: SeperatorDirection,
    on: Option<Color>,
    color: Option<Color>,
}
struct ColorizeTokenOptions {
    start: Option<DirectionOption>,
    end: Option<DirectionOption>,
    text: String,
    color: Option<Color>,
    on: Option<Color>,
    pad_string: bool,
}
fn create_seperated_formated_text(opts: ColorizeTokenOptions) -> String {
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

fn cap_tab_text(text: String, extra_space: usize, free_space_per_tab: usize) -> String {
    // Do we need to cut the tab name?
    let total_space = text.len() + extra_space;
    eprintln!("Is too large? text: {} free: {}", total_space, free_space_per_tab);
    if total_space > free_space_per_tab {
        eprintln!("Capping, size: {}, free space: {}, deriv: {}", total_space, free_space_per_tab, free_space_per_tab-3);
        return format!("{}...", text[..free_space_per_tab - 3].to_string());
    }
    return text;
}
fn fix_wonky_programming(cheat: usize) -> usize {
    // Because tab can be an uneven amount of characters.
    // Dividing by 2 will produce "the same" integer for n and n+1.
    // This has the affect that the line will "jump" every second character.
    // To witness this, uncomment the solution and just return the product.
    // eprintln!("val: {}", cheat);
    if cheat == 0 {
        return 0;
    }
    // eprintln!("val: {}, post: {}", cheat, ((cheat as f32) / 2.0).round() as usize);
    return ((cheat as f32) / 2.0).round() as usize;
}

fn create_session_name(maybe_session_name: Option<String>) -> ContentLol {
    if maybe_session_name.is_none() {
        return ContentLol {
            text: String::new(),
            number_of_chars: 0,
        };
    } else {
        let val = maybe_session_name.unwrap();
        let val_size = val.len();
        let x = create_seperated_formated_text(ColorizeTokenOptions {
            start: None,
            end: Some(DirectionOption {
                direction: SeperatorDirection::Right,
                color: Some(Color::Red),
                on: Some(Color::Black),
            }),
            on: Some(Color::Red),
            color: Some(Color::Black),
            text: val,
            pad_string: true,
        });

        return ContentLol {
            text: x,
            number_of_chars: val_size + 2 + 1,
        };
    }
}

fn calculate_left_padding(middle: usize, left_size: usize, center_size: usize) -> usize {
    if (middle - left_size) < center_size / 2 {
        return 0;
    }
    return middle - left_size - center_size / 2;
}

fn calculate_right_padding(
    middle: usize,
    right_size: usize,
    center_size: usize,
    left_padding_zero_compensation: usize,
    free_space: usize,
) -> usize {

    /*
    eprintln!(
        "Right padding: middle: {} right_size: {} center_size: {}, product: {}",
        middle,
        right_size,
        center_size,
        (middle - right_size - center_size)
    );
    */
    if left_padding_zero_compensation == 0 {
        return free_space;
    }
    if (middle - right_size) < center_size {
        return 0;
    }
    let calc = (middle - right_size - center_size);
    // Because it's offshot by 1.
    if calc > 0 {
        return calc-1;
    }
    return calc;
}

fn find_middle(cols: usize) -> usize {
    if cols % 2 != 0 {
        return (cols/2)+1;
    }
    return cols/2;
}
