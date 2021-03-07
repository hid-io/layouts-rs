use layouts::Layouts;

pub const USE_GITHUB: bool = true;

fn main() {
    if USE_GITHUB {
        let api_token = std::env::var("GITHUB_API_TOKEN").ok();
        let layouts = Layouts::from_github("hid-io/layouts".to_string(), "master", api_token);
        for layout in layouts.list_layouts() {
            println!("{}:\n{:?}\n", layout, layouts.get_layout(&layout));
        }
    } else {
        let layout_dir = "layouts";
        let layouts = Layouts::from_dir(layout_dir);
        for layout in layouts.list_layouts() {
            println!("{}:\n{:?}\n", layout, layouts.get_layout(&layout));
        }
    }
}
