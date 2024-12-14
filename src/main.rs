#![allow(dead_code)]
#![allow(unused)]

mod tab;

use crate::tab::Tab;

// Tortoise UI starting point
struct App {
    title_bar: TitleBar,
    panel: Panel,
}

// Default TitleBar should spawn one Tab instance, the behaviour can be changed
// from the settings.
struct TitleBar {
    // active_tab_uuid: u128
    tabs: Vec<Tab>,         // TODO: Active tab
    window_controls: bool,  // TODO: User Settings
                            //
                            // Not aestheticly pleasing but can be changed from
                            // the settings. If set to false, use default
                            // Windows titlebar.
}

// Create a new TitleBar instance, with the default tab.
impl TitleBar {
    fn new() -> Self {
        let mut tabs = Vec::new();
        tabs.push(Tab::default());

        Self {
            tabs,
            window_controls: true,
        }
    }
}

// Panel derives from the Tab instance, by default, spawns Main.
enum Panel {
    Main,       // Spawns: NavBar, SideBar, Content
    Custom,     // It's for custom content such as settings panel.
}

impl App {
    fn new() -> Self {
        Self {
            title_bar: TitleBar::new(),
            panel: Panel::Main,
        }
    }
}

fn main() {
}
