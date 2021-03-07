use layouts::Layouts;

fn main() {
    let layout_dir = "layouts";
    let layouts = Layouts::new(layout_dir);

    for layout in layouts.list_layouts() {
        println!("{}:\n{:?}\n", layout, layouts.get_layout(&layout));
    }
}
