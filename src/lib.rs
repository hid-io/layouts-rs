pub mod github;

#[macro_use]
extern crate derive_object_merge;

use github::GithubClient;
use glob::glob;
use object_merge::Merge;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

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
    pub fn from_file(file: PathBuf) -> Layout {
        let data = fs::read_to_string(&file).unwrap();
        Layout::from_str(&data).unwrap()
    }
}

impl FromStr for Layout {
    type Err = serde_json::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[derive(Debug, Clone)]
pub enum LayoutSource {
    Directory(PathBuf),
    Github(GithubClient, String),
}

pub fn list_dir(layout_dir: &str) -> Vec<String> {
    glob(&format!("{}/**/*.json", layout_dir))
        .unwrap()
        .filter_map(|path| path.ok())
        .map(|path| {
            path.strip_prefix(layout_dir)
                .unwrap()
                .to_string_lossy()
                .to_string()
        })
        .collect()
}

pub fn list_github(client: &GithubClient, reftag: &str) -> Vec<String> {
    client
        .list_files(reftag)
        .unwrap()
        .into_iter()
        .filter(|path| path.ends_with(".json"))
        .collect()
}

impl LayoutSource {
    pub fn list_layouts(&self) -> Vec<String> {
        match self {
            LayoutSource::Directory(dir) => list_dir(&dir.to_string_lossy()),
            LayoutSource::Github(client, reftag) => list_github(client, reftag),
        }
    }

    pub fn fetch_layout(&self, file: &str) -> Layout {
        //dbg!(file);
        match self {
            LayoutSource::Directory(dir) => Layout::from_file(dir.join(file)),
            LayoutSource::Github(client, reftag) => {
                let data = client.get_file_raw(file, reftag).unwrap();
                Layout::from_str(&data).unwrap()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Layouts {
    files: Vec<String>,
    cached_layouts: HashMap<String, Layout>,
    source: LayoutSource,
}

impl Layouts {
    pub fn new(source: LayoutSource) -> Self {
        Layouts {
            source,
            files: vec![],
            cached_layouts: HashMap::new(),
        }
    }

    pub fn from_dir(dir: PathBuf) -> Self {
        Layouts::new(LayoutSource::Directory(dir))
    }

    pub fn from_github(repo: String, reftag: String, api_token: Option<String>) -> Self {
        let client = GithubClient::new(repo, api_token);
        Layouts::new(LayoutSource::Github(client, reftag))
    }

    pub fn list_layouts(&mut self) -> Vec<String> {
        if self.files.is_empty() {
            self.files = self.source.list_layouts();
        }
        self.files.clone()
    }

    pub fn get_layout(&mut self, name: &str) -> Layout {
        if !self.cached_layouts.contains_key(name) {
            let layout = self.source.fetch_layout(name);
            self.cached_layouts.insert(name.to_string(), layout);
        }

        let layout = &self.cached_layouts[name];
        let parent = layout.parent.clone();

        let mut new_layout = layout.clone();
        if let Some(parent) = parent {
            new_layout.merge(&self.get_layout(&parent));
        }
        new_layout
    }
}

#[cfg(test)]
mod tests {
    use super::Layouts;
    use std::path::PathBuf;

    #[test]
    fn test_dir() {
        let layout_dir = PathBuf::from("layouts");
        let mut layouts = Layouts::from_dir(layout_dir);
        for layout in layouts.list_layouts() {
            println!("{}:\n{:?}\n", layout, layouts.get_layout(&layout));
        }
    }

    #[test]
    fn test_github() {
        let api_token = std::env::var("GITHUB_API_TOKEN");
        let mut layouts = Layouts::from_github(
            "hid-io/layouts".to_string(),
            "master".to_string(),
            api_token.ok(),
        );
        for layout in layouts.list_layouts() {
            println!("{}:\n{:?}\n", layout, layouts.get_layout(&layout));
        }
    }
}
