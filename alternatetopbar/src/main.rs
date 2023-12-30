mod seperator;
mod content;
mod types;

use colored::Colorize;
use zellij_tile::prelude::*;

extern crate alternatetopbar_lib;



#[derive(Default)]
struct State {
    tabs: Vec<TabInfo>,
    active_tab_idx: usize,
    mode_info: ModeInfo,
    plugin_conf: types::PluginConf,
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

        let left_conf = self.plugin_conf.left.clone();
        let right_conf = self.plugin_conf.right.clone();
        let center_conf = self.plugin_conf.center.clone();
        eprintln!("-----------------------");
        let left = content::config_to_actual_things(content::HugeConfig{
            mode_info: self.mode_info.clone(),
            direction: seperator::SeperatorDirection::Right,
            plugin_conf_dir: left_conf,
            free_space: cols,
            tabs: self.tabs.clone(),
        });
        let right = content::config_to_actual_things(content::HugeConfig{
            mode_info: self.mode_info.clone(),
            direction: seperator::SeperatorDirection::Left,
            plugin_conf_dir: right_conf,
            free_space: cols - left.number_of_chars,
            tabs: self.tabs.clone(),
        });
        let free_space_without_center = cols - right.number_of_chars - left.number_of_chars;
        let center = content::config_to_actual_things(content::HugeConfig{
            mode_info: self.mode_info.clone(),
            direction: seperator::SeperatorDirection::Right,
            plugin_conf_dir: center_conf,
            free_space: free_space_without_center,
            tabs: self.tabs.clone(),
        });

        // let session_name = create_session_name(session_name.clone());
        // let spot = create_fake_artist(seperator::SeperatorDirection::Right);
        // let left = merge_content(session_name, spot);
        // let mode = create_mode_part(mode);
        // let tabs = create_tabs(&self.tabs, free_space_without_center);
        if center.number_of_chars > cols {
            println!("Out of bounds");
            return;
        }

        let (no_left_padding, no_right_padding) = alternatetopbar_lib::find_padding(
            cols,
            left.number_of_chars,
            center.number_of_chars,
            right.number_of_chars,
        );
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
            "Cols: {}, Left size: {}, Tab size: {}, Right size: {}, left_padding: {left_padding}, right_padding: {right_padding}, free_space: {free_space}",
            cols,
            left.number_of_chars,
            center.number_of_chars,
            right.number_of_chars,
            left_padding = no_left_padding,
            right_padding = no_right_padding,
            free_space = free_space_without_center - center.number_of_chars,
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
            center = center.text,
            left = left.text,
            right = right.text,
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


