extern crate cursive;

use cursive::{
    Cursive,
    event::Key,
    view::*,
    views::*,
    traits::*
};
use std::fs::DirEntry;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::path::PathBuf;

fn main() {
    let mut siv = Cursive::default();

    let select = SelectView::<String>::new()
        .on_submit(on_submit)
        .with_id("select")
        .fixed_size((10, 5));

    let buttons = LinearLayout::vertical()
        .child(Button::new("Add", add))
        .child(Button::new("Delete", delete))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    siv.add_layer(Dialog::around(LinearLayout::horizontal()
        .child(select)
        .child(DummyView)
        .child(buttons))
        .title("Select a profile"));

    siv.run();

}

fn on_submit(s: &mut Cursive, name: &String) {
    s.pop_layer();
    s.add_layer(Dialog::text(format!("Name: {}\nAwesome: yes", name))
        .title("Hai")
        .button("Quit", Cursive::quit));
}

fn add(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_id("select", |view: &mut SelectView<String>| {
            view.add_item_str(name);
        });
        s.pop_layer();
    }
    s.add_layer(Dialog::around(EditView::new()
        .on_submit(ok)
        .with_id("name")
        .fixed_width(10))
        .title("Name plz")
        .button("OK", |s| {
            let name = s.call_on_id("name", |view: &mut EditView| {
                view.get_content()
            }).unwrap();
            ok(s, &name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }))
}

fn delete(s: &mut Cursive) {
    let mut select = s.find_id::<SelectView<String>>("select").unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No name to remove")),
        Some(focus) => {
            select.remove_item(focus);
        }
    }
}