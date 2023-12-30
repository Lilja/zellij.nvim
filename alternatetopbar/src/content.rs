use colored::Color;
use either::{Either, Left, Right};
use zellij_tile::prelude::*;

use crate::seperator;
use crate::types::{Part, PartType};

struct ContentLolWithNextColor {
    number_of_chars: usize,
    text: String,
    color: Color,
}
pub struct ContentLol {
    pub number_of_chars: usize,
    pub text: String,
}
pub fn merge_content(content1: ContentLol, content2: ContentLol, reversed: bool) -> ContentLol {
    let new_text = if !reversed {
        format!("{}{}", content1.text, content2.text)
    } else {
        format!("{}{}", content2.text, content1.text)
    };
    return ContentLol {
        text: new_text,
        number_of_chars: content1.number_of_chars + content2.number_of_chars,
    };
}

fn create_tabs(
    tabs: &Vec<TabInfo>,
    free_space: usize,
    direction: seperator::SeperatorDirection,
) -> ContentLolWithNextColor {
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
                        direction,
                    }),
                    end: Some(seperator::DirectionOption {
                        on: Some(Color::Black),
                        color: Some(Color::Yellow),
                        direction,
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
                        direction,
                    }),
                    end: Some(seperator::DirectionOption {
                        on: Some(Color::Black),
                        color: Some(Color::BrightBlack),
                        direction,
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

    return ContentLolWithNextColor {
        number_of_chars: chars,
        text: s,
        color: Color::Black,
    };
}

fn create_mode_part(
    mode: InputMode,
    shape: Box<dyn Fn(String, Option<Color>) -> seperator::ColorizeTokenOptions>,
) -> ContentLolWithNextColor {
    let zellij_human_readable_mode = format!("{:?}", mode).to_uppercase();

    let mode_color = if mode == InputMode::Locked {
        Color::Red
    } else {
        Color::Green
    };

    let size = zellij_human_readable_mode.len();
    let text = seperator::create_seperated_formated_text(shape(
        zellij_human_readable_mode,
        Some(mode_color),
    ));

    return ContentLolWithNextColor {
        // 2 spaces + 1 seperator
        number_of_chars: size + 2 + 1,
        text,
        color: mode_color,
    };
}

pub fn fix_wonky_programming(cheat: usize) -> usize {
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

fn create_session_name(
    maybe_session_name: Option<String>,
    shape: Box<dyn Fn(String, Option<Color>) -> seperator::ColorizeTokenOptions>,
) -> ContentLolWithNextColor {
    if maybe_session_name.is_none() {
        return ContentLolWithNextColor {
            text: String::new(),
            number_of_chars: 0,
            color: Color::Red,
        };
    } else {
        let session_name = maybe_session_name.unwrap();
        let val = format!("  {}", session_name);
        let val_size = session_name.len();
        // terminal icon + space
        let extra_space = 2 + 1;
        let x = seperator::create_seperated_formated_text(shape(val, None));

        return ContentLolWithNextColor {
            text: x,
            number_of_chars: val_size + extra_space + 2 + 1,
            color: Color::Red,
        };
    }
}

fn create_fake_artist(
    shape: Box<dyn Fn(String, Option<Color>) -> seperator::ColorizeTokenOptions>,
) -> ContentLolWithNextColor {
    let artist = "Russian Village Boys";
    let song = "Instababe";
    let artist_song_fmt = format!("{} - {}", artist, song);
    let final_fml = format!(" {}", artist_song_fmt);
    let val_size = artist_song_fmt.len();
    let extra_space = 2;
    let text = seperator::create_seperated_formated_text(shape(final_fml, None));

    return ContentLolWithNextColor {
        text,
        number_of_chars: val_size + extra_space + 2 + 1,
        color: Color::Magenta,
    };
}

pub struct HugeConfig {
    pub mode_info: ModeInfo,
    pub direction: seperator::SeperatorDirection,
    pub plugin_conf_dir: Vec<Part>,
    pub free_space: usize,
    pub tabs: Vec<TabInfo>,
}
pub fn config_to_actual_things(options: HugeConfig) -> ContentLol {
    let mut cl = ContentLol {
        text: String::new(),
        number_of_chars: 0,
    };
    let mut next_item_color: Option<Color> = None;
    let part_size = options.plugin_conf_dir.len();
    let temp_items = options.plugin_conf_dir.iter();
    let items = if options.direction == seperator::SeperatorDirection::Right {
        Left(temp_items.rev().enumerate())
    } else {
        Right(temp_items.enumerate())
    };
    let is_reversed = options.direction == seperator::SeperatorDirection::Right;
    for (idx, item) in items {
        let direction = options.direction.clone();
        let rev_idx = reversed_index(is_reversed, idx, part_size);
        let is_start = rev_idx == 0;
        let is_end = rev_idx + 1 == part_size;
        let part_type = item.part_type.clone();
        let shape = construct_shape(is_start, is_end, item.clone(), next_item_color, direction);
        let content = match part_type {
            PartType::Session => create_session_name(options.mode_info.session_name.clone(), shape),
            PartType::Spotify => create_fake_artist(shape),
            PartType::Tabs => create_tabs(&options.tabs, options.free_space, direction),
            PartType::Mode => create_mode_part(options.mode_info.mode, shape),
        };
        next_item_color = Some(content.color);
        cl = merge_content(
            cl,
            ContentLol {
                text: content.text,
                number_of_chars: content.number_of_chars,
            },
            is_reversed,
        );
    }
    return cl;
}

fn construct_shape(
    is_start: bool,
    is_end: bool,
    part: Part,
    transition_color: Option<Color>,
    direction: seperator::SeperatorDirection,
) -> Box<dyn Fn(String, Option<Color>) -> seperator::ColorizeTokenOptions> {
    match direction {
        // Left side, LTR
        seperator::SeperatorDirection::Right => Box::new(move |text, custom_bg_color| {
            eprintln!("{}", text);
            eprintln!("Is start? {}, is end? {}", is_start, is_end);
            eprintln!(
                "transition_color {}, custom {}",
                transition_color.is_some(),
                custom_bg_color.is_some()
            );
            let mut start: Option<seperator::DirectionOption> = None;
            /*
            if !is_start {
                start = Some(seperator::DirectionOption {
                    direction,
                    on: custom_bg_color.or(Some(bg_color)),
                    color: Some(Color::Black),
                })
            }
            */
            let end;
            if transition_color.is_some() {
                end = Some(seperator::DirectionOption {
                    direction,
                    on: transition_color,
                    color: custom_bg_color.or(Some(part.bg_color)),
                });
            } else {
                end = Some(seperator::DirectionOption {
                    direction,
                    on: custom_bg_color,
                    color: transition_color.or(Some(part.bg_color)),
                });
            }

            seperator::ColorizeTokenOptions {
                start,
                end,
                text,
                color: Some(Color::Black),
                on: custom_bg_color.or(Some(part.bg_color)),
                pad_string: true,
            }
        }),
        // Right side, RTL
        seperator::SeperatorDirection::Left => Box::new(move |text, custom_bg_color| {
            let end: Option<seperator::DirectionOption> = None;
            let start = Some(seperator::DirectionOption {
                direction,
                on: transition_color.or(Some(Color::Black)),
                color: custom_bg_color.or(Some(part.bg_color)),
            });
            seperator::ColorizeTokenOptions {
                start,
                end,
                text,
                color: Some(Color::Black),
                on: custom_bg_color.or(Some(part.bg_color)),
                pad_string: true,
            }
        }),
    }
}

fn reversed_index(reversed: bool, index: usize, size: usize) -> usize {
    if reversed {
        return size - (index + 1);
    }
    return index;
}
