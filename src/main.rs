#![allow(dead_code)]
#![allow(unused)]

mod tab;

use crate::tab::Tab;
use std::u128;

// Tortoise UI starting point
#[derive(Debug)]
struct App {
    title_bar: TitleBar,
    // panel: Panel,
}

// Default TitleBar should spawn one Tab instance, the behaviour can be changed
// from the settings.
#[derive(Debug)]
struct TitleBar {
    active_tab: u128,
    tabs: Vec<Tab>,
    window_controls: bool, // Should be handled elsewhere
}

// Create a new TitleBar instance, with the default tab.
impl TitleBar {

    // TODO: Handle with Result<>
    fn set_active(&mut self, uuid: u128) {
        if self.tabs.iter().any(|tab| tab.uuid == uuid) {
            self.active_tab = uuid;
        }
    }

    fn new() -> Self {
        let default_tab = Tab::default();
        //
        // EXPERIMENTAL, TODO: Introduce Lifetimes
        //
        let active_tab = default_tab.uuid.clone();

        let mut tabs = Vec::new();
        tabs.push(default_tab);

        Self {
            tabs,
            window_controls: true,
            active_tab,
        }
    }

    //
    // nocheckin
    // nocheckin
    // nocheckin
    // nocheckin
    // nocheckin
    // Logic is shit, introduce lifetimes asap.
    // TODO: Lifetimes...
    fn add_tab(&mut self, new_tab: Tab) {
        let new_tab_uuid = new_tab.uuid.clone();
        self.tabs.push(new_tab);
        self.set_active(new_tab_uuid);
    }
}

// Panel derives from the Tab instance, by default, spawns Main.
// enum Panel {
//     Main,       // Spawns: NavBar, SideBar, Content
//     Custom,     // It's for custom content such as settings panel.
// }

impl App {
    fn new() -> Self {
        Self {
            title_bar: TitleBar::new(),
            // panel: Panel::Main,
        }
    }
}

fn main() {
    let mut tortoise = App::new();
    //
    // Hardode Settings Tab
    //
    let settings_tab = Tab {
        path: "".into(),
        icon: "SETTINGS ICON".into(),
        label: String::from("Settings"),
        custom: true,
        uuid: 1,
    };

    tortoise.title_bar.add_tab(settings_tab);
    dbg!(tortoise);
}
