use std::path::Path;

// Should be hot-reloadable.
#[derive(Debug)]
pub struct Tab {
    // Instead of complicating it, just use an empty String for custom tabs (for
    // now).
    pub path: String,

                        // TODO: rename to icon_path
    pub icon: String,   // `special_icon` || `default_icon` || `custom_icon`

    pub label: String,  // Last sorted regex of the path string or custom label
    pub custom: bool,
    pub uuid: std::primitive::u128,
}

// custom: false
impl Tab {
    // TODO:
    // If the path is in special_directories return corresponding
    // `special_icon`, otherwise return `default_icon`.
    fn get_icon() -> String {
        "DOWNLOADS ICON".into()
    }

    // Generate label from path. Filter the path and return the last part.
    fn generate_label(path: &str) -> String {
        Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(path)
        .to_string()
    }

    // TODO: Retrieve the default_path from user settings.
    fn get_default_path() -> String {
        "C:\\Users\\ilhan\\Downloads".into()
    }
}

// @Redundant
// custom: true
// impl Tab {
//     pub fn spawn_settings_tab() -> Self {
//         Self {
//             path: "".into(),
//             icon: "SETTINGS ICON".into(), // TODO
//             label: String::from("Settings"),
//             custom: true,
//             uuid: 1,
//         }
//     }
// }

impl Default for Tab {
    fn default() -> Self {

        let default_path = Self::get_default_path();

        Self {
            label: Self::generate_label(&default_path),
            icon: Self::get_icon(),
            custom: false,

            path: default_path,
            // 
            // We probably need a hardcoded UUID for the default directory,
            // stupid to create a different uuid everytime its called.
            //
            uuid: 0,
        }
    }
}

