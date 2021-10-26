use layouts_rs::Layouts;
use std::path::PathBuf;

pub const USE_GITHUB: bool = true;
pub const LOCAL_DIR: &str = "layouts";
pub const GIT_REPO: &str = "hid-io/layouts";
pub const GIT_BRANCH: &str = "master";

fn main() {
    let mut layouts = if USE_GITHUB {
        let api_token = std::env::var("GITHUB_API_TOKEN");
        Layouts::from_github(GIT_REPO.to_string(), GIT_BRANCH.to_string(), api_token.ok())
    } else {
        Layouts::from_dir(PathBuf::from(LOCAL_DIR))
    };

    let layout = "keyboards/en_US.json";
    println!("{}:\n{:?}\n", layout, layouts.get_layout(layout));
}
