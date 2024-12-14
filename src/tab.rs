// Should be hot-reloadable.
pub struct Tab {
    // Instead of complicating it, just use an empty String for custom tabs (for
    // now).
    pub path: String,

                        // TODO: rename to icon_path
    pub icon: String,   // `special_icon` || `default_icon` || `custom_icon`

    pub label: String,  // Last sorted regex of the path string or custom label
    pub custom: bool,
}

// custom: false
impl Tab {
    // If the path is in special_directories return corresponding
    // `special_icon`, otherwise return `default_icon`.
    fn get_icon() -> String {
        todo!()
    }

    // Generate label from path. Filter the path and return the last part.
    fn generate_label(path: &str) -> String {
        todo!()
    }

    // Retrieve the default_path from user settings.
    fn get_default_path() -> String {
        todo!()
    }

    // @Redundant?
    // fn is_custom(&mut self) -> bool {
    //     self.custom
    // }
}

impl Default for Tab {
    fn default() -> Self {

        let default_path = Self::get_default_path();

        Self {
            label: Self::generate_label(&default_path),
            icon: Self::get_icon(),
            custom: false,

            path: default_path,
        }
    }
}

// custom: true
impl Tab {
    fn spawn_settings_tab() -> Self {
        Self {
            path: "".into(),
            icon: todo!(), // TODO: Set settings icon.
            label: String::from("Settings"),
            custom: true,
        }
    }
}
