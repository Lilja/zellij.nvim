use colored::Colorize;
use regex::Regex;
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
        let sep = "\u{E0B0}";
        let mut s = String::new();

        self.tabs.iter().enumerate().for_each(|(idx, tab)| {
            if tab.active {
                s.push_str(&format!(
                    "{}{}{}",
                    sep.on_yellow().black(),
                    format!(" {} ", tab.name).on_yellow().black(),
                    sep.yellow()
                ));
                s.push_str(&sep.red().on_black());
                s.push_str(&("yalla".blue()));
            } else {
                s.push_str(&format!(
                    "{}{}",
                    sep.on_bright_black().black(),
                    format!(" {} ", tab.name).on_bright_black().on_bright_black(),
                ));
                s.push_str(&sep.on_bright_black().bright_black());
            }
            // s.push_str(&" ");
            // if self.tabs.len() > idx+1
        });
        println!("Hi mom! rows:{} cols:{}{}", rows, cols, s);
    }
}
