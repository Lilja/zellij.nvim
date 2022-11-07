use colored::Colorize;
use regex::Regex;
use zellij_tile::prelude::*;
use unicode_segmentation::UnicodeSegmentation;


#[derive(Default)]
struct State {
    tabs: Vec<TabInfo>,
    active_tab_idx: usize,
    mode_info: ModeInfo,
    //mouse_click_pos: usize,
    //should_render: bool,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        set_selectable(false);
        subscribe(&[EventType::TabUpdate, EventType::ModeUpdate])
    }

    fn update(&mut self, event: Event) {
        match event {
            Event::ModeUpdate(mode_info) => self.mode_info = mode_info,
            Event::TabUpdate(tabs) => {
                self.active_tab_idx = tabs.iter().position(|t| t.active).unwrap() + 1;
                let mut new_tabs: Vec<TabInfo> = vec![];
                for (i, tab) in tabs.iter().enumerate() {
                    let temp = TabInfo {
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
            }
            _ => {}
        }
    }

    fn render(&mut self, rows: usize, cols: usize) {
        let left_sep = "\u{E0B0}";
        let right_sep = "\u{E0B2}";
        let mut s = String::new();

        self.tabs.iter().enumerate().for_each(|(idx, tab)| {
            if tab.active {
                s.push_str(&format!(
                    "{}{}{}",
                    left_sep.on_yellow().black(),
                    format!(" {} ", tab.name).on_yellow().black(),
                    left_sep.yellow()
                ));
                // s.push_str(&format!("{}", leftSep.red().on_black()));
            } else {
                s.push_str(&format!(
                    "{}{}",
                    left_sep.on_bright_black().black(),
                    format!(" {} ", tab.name).on_bright_black(),
                ));
                s.push_str(&format!("{}", left_sep.bright_black()));
            }
            // s.push_str(&" ");
            // if self.tabs.len() > idx+1
        });
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
        let text_size: usize = self.tabs.iter().map(|i| i.name.len() + 5).sum();
        let col_diff = cols - text_size;
        let zellij_human_readable_mode = format!("{:?}", mode).to_uppercase();
        let zellij_human_readable_mode_padded = format!("{} ", zellij_human_readable_mode).black().on_green();
        let left = "";
        let right = format!(
            "{right_sep_uncolored}{right_sep_colored}{zellijHumanReadableModePadded}",
            right_sep_uncolored=right_sep.green().on_black(),
            right_sep_colored=right_sep.green().on_green(),
            zellijHumanReadableModePadded=zellij_human_readable_mode_padded,

        );

        let left_padding = " ".repeat((col_diff-left.len()) / 2).on_black();
        let right_padding = " ".repeat((col_diff-(zellij_human_readable_mode.len()+3)) / 2).on_black();
        println!("{} {} {} {}", cols, col_diff, zellij_human_readable_mode.len(), (col_diff-(zellij_human_readable_mode.len()+3)));

        println!(
            "{left}{left_padding}{center}{right_padding}{right}",
             left_padding=left_padding,
             right_padding=right_padding,
             center=s,
             left=left,
             right=right,
         );
    }
}
