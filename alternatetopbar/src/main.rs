mod seperator;
mod spacing;

use colored::{Color, Colorize};
use zellij_tile::prelude::*;

extern crate alternatetopbar_lib;

#[derive(Default)]
struct State {
    tabs: Vec<TabInfo>,
    active_tab_idx: usize,
    mode_info: ModeInfo,
    //mouse_click_pos: usize,
}

register_plugin!(State);

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
                for tab in tabs.iter() {
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
        if tabs.number_of_chars > cols {
            println!("Out of bounds");
            return;
        }
        let middle = spacing::find_middle(cols);

        let free_space = if tabs.number_of_chars > free_space_without_center {
            0
        } else {
            free_space_without_center - tabs.number_of_chars
        };
        let tab_size = fix_wonky_programming(tabs.number_of_chars);
        let (no_left_padding, no_right_padding) = alternatetopbar_lib::find_padding(cols, left.number_of_chars, tabs.number_of_chars, mode.number_of_chars);
        /*
        let no_left_padding =
            spacing::calculate_left_padding(middle, left.number_of_chars, tabs.number_of_chars);
        */
        let left_padding = " ".repeat(no_left_padding).on_black();
        /*
        let no_right_padding = spacing::calculate_right_padding(
            cols,
            middle,
            mode.number_of_chars,
            tab_size,
            no_left_padding,
            free_space,
        );
        */
        let right_padding = " ".repeat(no_right_padding).on_black();
        /*
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
        */
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
            center = tabs.text,
            left = left.text,
            right = mode.text,
        );
        /*
        println!(
            "{left}{m}{right}",
            left = "*".repeat(middle - 1),
            m = "I".yellow(),
            right = "*".repeat(if cols % 2 == 0 { middle } else { middle - 1 })
        );
        println!(
            "left   size: {}, padding: {}",
            left.number_of_chars, no_left_padding
        );
        println!(
            "center size: {}, divided and rounded: {}.",
            tabs.number_of_chars, tab_size
        );
        println!(
            "right  size: {}, padding: {}",
            mode.number_of_chars, no_right_padding
        );
        println!("cols: {}, middle: {}, free_space_without_center: {}, free space: {}", cols, middle, free_space_without_center, free_space);
        */
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
        let capped_text = seperator::cap_tab_text(
            name.to_string(),
            extra_space_per_tab,
            free_space / number_of_tabs,
        );
        let capped_text_size = capped_text.len();
        // eprintln!("New text size: {}", capped_text_size);
        if tab.active {
            s.push_str(&seperator::create_seperated_formated_text(
                seperator::ColorizeTokenOptions {
                    start: Some(seperator::DirectionOption {
                        on: Some(Color::Yellow),
                        color: Some(Color::Black),
                        direction: seperator::SeperatorDirection::Right,
                    }),
                    end: Some(seperator::DirectionOption {
                        on: Some(Color::Black),
                        color: Some(Color::Yellow),
                        direction: seperator::SeperatorDirection::Right,
                    }),
                    text: capped_text,
                    on: Some(Color::Yellow),
                    color: Some(Color::Black),
                    pad_string: true,
                },
            ));
        } else {
            s.push_str(&seperator::create_seperated_formated_text(
                seperator::ColorizeTokenOptions {
                    start: Some(seperator::DirectionOption {
                        on: Some(Color::BrightBlack),
                        color: Some(Color::Black),
                        direction: seperator::SeperatorDirection::Right,
                    }),
                    end: Some(seperator::DirectionOption {
                        on: Some(Color::Black),
                        color: Some(Color::BrightBlack),
                        direction: seperator::SeperatorDirection::Right,
                    }),
                    text: capped_text,
                    on: Some(Color::BrightBlack),
                    color: None,
                    pad_string: true,
                },
            ));
        }
        // Space left and right of tab name
        chars = chars + 2;
        // Tab name itself
        chars = chars + capped_text_size;
        // Left and right separator
        chars = chars + 2;
    });

    return ContentLol {
        number_of_chars: chars,
        text: s,
    };
}

fn create_mode_part(mode: &InputMode) -> ContentLol {
    let zellij_human_readable_mode = format!("{:?}", mode).to_uppercase();

    let size = zellij_human_readable_mode.len();
    let text = seperator::create_seperated_formated_text(seperator::ColorizeTokenOptions {
        start: Some(seperator::DirectionOption {
            direction: seperator::SeperatorDirection::Left,
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
        let session_name = maybe_session_name.unwrap();
        let val = format!(" {}", session_name);
        let val_size = session_name.len();
        // terminal icon + space
        let extra_space = 1+1;
        let x = seperator::create_seperated_formated_text(seperator::ColorizeTokenOptions {
            start: None,
            end: Some(seperator::DirectionOption {
                direction: seperator::SeperatorDirection::Right,
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
            number_of_chars: val_size + extra_space + 2 + 1,
        };
    }
}
