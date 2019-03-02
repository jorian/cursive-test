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
use std::rc::Rc;
use std::path::PathBuf;

fn main() {
    let mut siv = Cursive::default();

    let mut panes = LinearLayout::horizontal();
    let picker = file_picker(".")
        .with_id("picker");
    panes.add_child(picker.fixed_size((30, 25)));
    panes.add_child(DummyView);
    panes.add_child(TextView::new("file contents")
        .with_id("contents")
        .scrollable()
        .fixed_size((65, 75)));
    let mut layout = LinearLayout::vertical();
    layout.add_child(panes);
    layout.add_child(TextView::new("status")
        .with_id("status")
        .fixed_size((40, 1)));
    siv.add_layer(Dialog::around(layout)
        .button("Quit", |a| a.quit())
        .button("Back", back));
    siv.run();
}

fn back(s: &mut Cursive) {
    let mut picker = s.find_id::<SelectView<PathBuf>>("picker").unwrap();
    let dir: Rc<PathBuf> = picker.selection().unwrap();

    let path = dir.clone().parent().unwrap().parent().unwrap().to_path_buf();
    dbg!(&path);

    picker.clear();

    for entry in fs::read_dir(path).expect("Unable to read") {
        if let Ok(e) = entry {
            let file_name = e.file_name().into_string().unwrap();
            picker.add_item(file_name, e.path());
        }
    }
}

fn file_picker<D: AsRef<Path>>(directory: D) -> SelectView<PathBuf> {
    let mut view = SelectView::new();
    for entry in fs::read_dir(directory).expect("Unable to read") {
        if let Ok(e) = entry {
            let file_name = e.file_name().into_string().unwrap();
            view.add_item(file_name, e.path());
        }
    }
    // when selecting a file, update statusbar
    // when clicking a file, load the contents in other pane:
    view.on_select(update_status).on_submit(submit_choice)
}

fn submit_choice(s: &mut Cursive, entry: &PathBuf) {
    if entry.is_dir() {
        let mut picker = s.find_id::<SelectView<PathBuf>>("picker").unwrap();

        let dir: Rc<PathBuf> = picker.selection().unwrap();
        let dir = String::from(dir.as_os_str().to_str().unwrap());

        dbg!(&dir);
        picker.clear();

        for entry in fs::read_dir(dir).expect("Unable to read") {
            if let Ok(e) = entry {
                let file_name = e.file_name().into_string().unwrap();
                picker.add_item(file_name, e.path());
            }
        }
    } else {
        let mut text_view = s.find_id::<TextView>("contents").unwrap();
        let mut buf = String::new();
        dbg!(&entry.to_str().unwrap());

        let _ = File::open(entry)
            .and_then(|mut f| f.read_to_string(&mut buf))
            .map_err(|e| Dialog::info(format!("Error: {}", e.to_string())));
        text_view.set_content(buf)
    }
}

fn update_status(s: &mut Cursive, entry: &PathBuf) {
    let mut status_bar = s.find_id::<TextView>("status").unwrap();
    let file_name = entry.to_str().unwrap();
    dbg!(&file_name);
    let file_size = entry.metadata().unwrap().len();
    let content = format!("{}: {} bytes", file_name, file_size);
    dbg!(&content);
    status_bar.set_content(content);
}