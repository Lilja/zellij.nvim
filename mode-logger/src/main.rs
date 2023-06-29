use colored::{Colorize, ColoredString, Color};
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

const LEFT_SEP: &str = "\u{E0B0}";
const RIGHT_SEP: &str = "\u{E0B2}";

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
        let lol = create_tabs(&self.tabs);
        /*
         * <-----------------------------> col     - e.g. 160
         * <--------------|--------------> middle = col / 2
         * <text>                          s.len() - e.g. 60
         * <text><-----------------------> padding needed = col - s.len()
         *       <------------>            padding needed left side = padding needed/2
         *
         * <-----------><text><---------->
         */

        /**
         * <-----------------------------> col     - e.g. 160
         *             <Normal>            Mode part (<No>)
         *          <Tab 1, Tab 2>         Tab display (<Ta>)
         *             <Session>           Zellij session name (<Se>)
         * <--------------|--------------> middle = col / 2
         * <Se><-------><Ta>               left padding = middle - len(<Left>) - len(<Middle>)
         *              <Ta><--------><Se> right padding = middle - len(<Righ>) - len(<Middle>)
         *
         * */
        let total_unused_space = cols - lol.number_of_chars;
        let middle = cols / 2;
        let lol2 = create_mode_part(&self.mode_info.mode);
        let left = "";

        let left_padding = " ".repeat((total_unused_space - left.len()) / 2).on_black();
        let right_padding = " "
            .repeat(5)
            .on_black();
        println!(
            "Cols: {} total_unused_space: {} zhrm: {} mode space: {}, tab space: {}",
            cols,
            total_unused_space,
            lol2.text.len(),
            lol2.number_of_chars,
            lol.number_of_chars,
        );
        /*
        println!(
            "Left padding {}, Right padding: {}",
            (total_unused_space - left.len()) / 2,
            (total_unused_space - (zellij_human_readable_mode.len() + 3)) / 2,
        );
        */

        print!(
            "{left}{left_padding}{center}{right_padding}{right}",
            left_padding = left_padding,
            right_padding = right_padding,
            center = lol.text,
            left = left,
            right = lol2.text,
        );
    }
}

struct ContentLol {
    number_of_chars: usize,
    text: String,
}

fn create_tabs(tabs: &Vec<TabInfo>) -> ContentLol {
    let mut s = String::new();
    let mut chars: usize = 0;

    tabs.iter().for_each(|tab| {
        let name = &tab.name;
        if tab.active {
            s.push_str(&format!(
                "{}{}{}",
                LEFT_SEP.on_yellow().black(),
                format!(" {} ", name).on_yellow().black(),
                LEFT_SEP.yellow()
            ));

            // s.push_str(&format!("{}", leftSep.red().on_black()));
        } else {
            /*
            s.push_str(&format!(
                "{}{}",
                LEFT_SEP.on_bright_black().black(),
                format!(" {} ", name).on_bright_black(),
            ));
            s.push_str(&format!("{}", LEFT_SEP.bright_black()));
            */

            s.push_str(&create_cool_text(ColorizeTokenOptions{
                start: Some(DirectionOption {
                    on: Some(Color::BrightBlack),
                    color: Some(Color::Black),
                    direction: SeperatorDirection::Left,
                }),
                end: Some(DirectionOption {
                    on: Some(Color::BrightBlack),
                    color: None,
                    direction: SeperatorDirection::Left,
                }),
                text: name.to_string(),
                on: Some(Color::BrightBlack),
                color: None,
                pad_string: true,

            }));
        }
        // Space left and right of tab name
        chars = chars + 2;
        // Tab name itself
        chars = chars + tab.name.len();
        // Left and right separator
        chars = chars + 2;
        // s.push_str(&" ");
        // if self.tabs.len() > idx+1
    });

    let number_of_spaces_between_tabs: usize = tabs.len() - 1;

    return ContentLol {
        number_of_chars: chars + number_of_spaces_between_tabs,
        text: s,
    };
}

fn create_mode_part(mode: &InputMode) -> ContentLol {
    let zellij_human_readable_mode = format!("{:?}", mode).to_uppercase();
    let zellij_human_readable_mode_padded = format!(" {} ", zellij_human_readable_mode)
        .black()
        .on_green();
    let right = format!(
        "{right_sep_uncolored}{zellijHumanReadableModePadded}",
        right_sep_uncolored = RIGHT_SEP.green().on_black(),
        zellijHumanReadableModePadded = zellij_human_readable_mode_padded,
    );

    return ContentLol {
        // 2 spaces + 1 seperator
        number_of_chars: zellij_human_readable_mode.len() + 2 + 1,
        text: right,
    }
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
fn create_cool_text(opts: ColorizeTokenOptions) -> String {
    let output = color_token(&opts.text, opts.color, opts.on, opts.pad_string);
    let mut _start = String::new();
    let mut _end = String::new();
    match opts.start {
        Some(start) => {
            _start = color_token(
                if start.direction == SeperatorDirection::Left { LEFT_SEP } else { RIGHT_SEP },
                start.color,
                start.on,
                false
            )
        },
        None => {},
    }
    match opts.end {
        Some(end) => {
            let sep = if end.direction == SeperatorDirection::Left { LEFT_SEP } else { RIGHT_SEP };
            _end = color_token(
                sep,
                end.color,
                end.on,
                false,
            );
        },
        None => {},
    }
    return format!("{}{}{}", _start, output, _end);
}

fn color_token(text: &str, fg: Option<Color>, bg: Option<Color>, pad_string: bool) -> String {
    let mut output = String::new();
    let padded_output = if pad_string { format!(" {} ", text) } else { format!("{}", text) };


    let aaaa = if fg.is_some() { padded_output.color(fg.unwrap()) } else { padded_output };
    match fg {
        Some(a) => {
            output = format!("{}", padded_output.color(a))
        },
        None => {},
    }

    match bg {
        Some(b) => {
            output = format!("{}{}", output, padded_output.on_color(b));
        }
        None => {},
    }
    return output;
}
