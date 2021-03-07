#[macro_use]
extern crate derive_object_merge;

use glob::glob;
use object_merge::Merge;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Header {
    pub name: Vec<String>,
    pub authors: Vec<String>,
    pub notes: Vec<String>,
    pub locale: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Merge)]
#[serde(default)]
pub struct Layout {
    #[shallow_merge]
    #[serde(flatten)]
    pub header: Option<Header>, // don't merge
    pub hid_locale: Option<String>,
    pub parent: Option<String>,
    #[combine]
    pub locale_notes: Vec<String>,
    pub to_hid_locale: HashMap<String, String>,
    pub from_hid_locale: HashMap<String, String>,
    #[combine]
    pub keyboard_notes: Vec<String>,
    pub to_hid_keyboard: HashMap<String, String>,
    pub from_hid_keyboard: HashMap<String, String>,
    #[combine]
    pub led_notes: Vec<String>,
    pub to_hid_led: HashMap<String, String>,
    pub from_hid_led: HashMap<String, String>,
    #[combine]
    pub sysctrl_notes: Vec<String>,
    pub to_hid_sysctrl: HashMap<String, String>,
    pub from_hid_sysctrl: HashMap<String, String>,
    #[combine]
    pub consumer_notes: Vec<String>,
    pub to_hid_consumer: HashMap<String, String>,
    pub from_hid_consumer: HashMap<String, String>,
    #[combine]
    pub composition: HashMap<String, Vec<Vec<String>>>,
}

impl Layout {
    pub fn new(file: PathBuf) -> Layout {
        let data = fs::read_to_string(&file).unwrap();
        serde_json::from_str(&data).unwrap()
    }
}

pub struct Layouts {
    layouts: HashMap<String, Layout>,
}

impl Layouts {
    pub fn new(layout_dir: &str) -> Self {
        let mut layouts = HashMap::new();

        let file_paths = glob(&format!("{}/**/*.json", layout_dir))
            .unwrap()
            .filter_map(|x| x.ok());
        for path in file_paths {
            let name = path
                .clone()
                .strip_prefix(layout_dir)
                .unwrap()
                .to_string_lossy()
                .to_string();
            layouts.insert(name, Layout::new(path));
        }

        Layouts { layouts }
    }

    pub fn list_layouts(&self) -> Vec<String> {
        self.layouts.keys().cloned().collect()
    }

    pub fn get_layout(&self, name: &str) -> Layout {
        let layout = &self.layouts[name];

        let mut new_layout = layout.clone();
        if let Some(parent) = &layout.parent {
            new_layout.merge(&self.get_layout(parent));
        }
        new_layout
    }
}
