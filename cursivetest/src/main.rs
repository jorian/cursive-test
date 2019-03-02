extern crate cursive;

use cursive::{
    Cursive,
    event::Key,
    view::*,
    views::*};
use std::fs::DirEntry;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut siv = Cursive::default();

    let mut boxes = LinearLayout::horizontal();
    let picker = file_picker(".")
        .with_id("picker");
    boxes.add_child(picker.fixed_size((30, 25)));
    boxes.add_child(DummyView);
    boxes.add_child(TextView::new("file contents")
        .with_id("contents")
        .scrollable()
        .fixed_size((65, 75)));
    let mut layout = LinearLayout::vertical();
    layout.add_child(boxes);
    layout.add_child(TextView::new("status")
        .with_id("status")
        .fixed_size((80, 1)));
    siv.add_layer(Dialog::around(layout).button("Quit", |a| a.quit()));
    siv.run();
}

fn file_picker<D: AsRef<Path>>(directory: D) -> SelectView<DirEntry> {
    let mut view = SelectView::new();
    for entry in fs::read_dir(directory).expect("Unable to read") {
        if let Ok(e) = entry {
            let file_name = e.file_name().into_string().unwrap();
            view.add_item(file_name, e);
        }
    }
    // when selecting a file, update statusbar
    // when clicking a file, load the contents in other pane:
    view.on_select(update_status).on_submit(load_contents)
}

fn update_status(s: &mut Cursive, entry: &DirEntry) {
    let mut status_bar = s.find_id::<TextView>("status").unwrap();
    let file_name = entry.file_name().into_string().unwrap();
    let file_size = entry.metadata().unwrap().len();
    let content = format!("{}: {} bytes", file_name, file_size);
    status_bar.set_content(content);
}

fn load_contents(s: &mut Cursive, entry: &DirEntry) {
    let mut text_view = s.find_id::<TextView>("contents").unwrap();
    let content = if entry.metadata().unwrap().is_dir() {
        s.call_on_id("picker", |view: &mut SelectView<DirEntry>| {
            dbg!(&view.selection().unwrap());
            let dir = view.selection().unwrap();
            
        });
        String::from("<DIR>")
    } else {
        let mut buf = String::new();
        let _ = File::open(entry.file_name())
            .and_then(|mut f| f.read_to_string(&mut buf))
            .map_err(|e| buf = format!("Error: {}", e));
        buf
    };
    text_view.set_content(content)
}