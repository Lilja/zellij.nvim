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
    //should_render: bool,
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

        let mode = create_mode_part(&self.mode_info.mode);
        let free_space = cols - mode.number_of_chars;
        let tabs = create_tabs(&self.tabs, free_space);
        let total_unused_space = cols - tabs.number_of_chars;
        if tabs.number_of_chars > cols {
            println!("Out of bounds");
            return;
        }
        let middle = cols / 2;
        let left = "";

        let tab_size = fix_wonky_programming(tabs.number_of_chars-4);
        let left_padding = " "
            .repeat(middle - left.len() - tabs.number_of_chars/2)
            .on_black();
        let right_padding = " "
            .repeat(middle - mode.number_of_chars - tab_size)
            .on_black();
        println!(
            "Cols: {}, Middle: {}, Tabs: {}, Mode: {}, right_padding: {}",
            cols,
            middle,
            tabs.number_of_chars,
            mode.number_of_chars,
            middle - mode.number_of_chars - tab_size,
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
            left = left,
            right = mode.text,
        );
        print!("{left}{m}", left = " ".repeat(middle - 1), m = "I".yellow());
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
        let capped_text = cap_tab_text(name.to_string(), free_space / number_of_tabs);
        let capped_text_size = capped_text.len();
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

fn cap_tab_text(text: String, free_space_per_tab: usize) -> String {
    // Do we need to cut the tab name?
    if text.len() > free_space_per_tab {
        return format!("{}...", text[..free_space_per_tab].to_string());
    }
    return text;
}
fn fix_wonky_programming(cheat: usize) -> usize {
    // Because tab can be an uneven amount of characters.
    // Dividing by 2 will produce "the same" integer for n and n+1.
    // This has the affect that the line will "jump" every second character.
    // To witness this, uncomment the solution and just return the product.
    eprintln!("val: {}", cheat);
    if cheat == 0 {
        return 0;
    }
    eprintln!("val: {}, post: {}", cheat, (cheat+1)/2);
    return (cheat+1) / 2;
}

