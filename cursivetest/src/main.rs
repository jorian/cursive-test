extern crate cursive;

use cursive::{
    Cursive,
    event::Key,
    view::*,
    views::*};

fn main() {
    let mut siv = Cursive::default();

    let selectv = SelectView::<String>::new()
        .on_submit(select_name)
        .with_id("selectv")
        .fixed_size((20, 10));

    let buttons = LinearLayout::vertical()
        .child(Button::new("Add", add_name))
        .child(Button::new("Remove", remove_name))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    siv.add_layer(Dialog::around(LinearLayout::horizontal()
        .child(selectv)
        .child(DummyView)
        .child(buttons))
        .title("Select a profile"));

    siv.run();
}

fn select_name(s: &mut Cursive, name: &String) {
    s.pop_layer();
    s.add_layer(Dialog::text(format!("Name: {}\nAwesome: yes", name))
        .title(format!("{}'s info", name))
        .button("Quit", Cursive::quit));
}

fn add_name(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_id("selectv", |view: &mut SelectView<String>| {
            view.add_item_str(name);
        });
        s.pop_layer();
    }

    s.add_layer(Dialog::around(
        EditView::new()
            .on_submit(ok)
            .with_id("name")
            .fixed_width(10))
        .title("Enter a name")
        .button("Ok", |e| {
            let name = e.call_on_id("name", |view: &mut EditView| {
                view.get_content()
            }).unwrap();
            ok(e, &name);
        })
        .button("Cancel", |e| { e.pop_layer(); })) // close popup

}

fn remove_name(s: &mut Cursive) {
    let mut select = s.find_id::<SelectView<String>>("selectv").unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No name to remove")),
        Some(focus) => {
            select.remove_item(focus);
        }
    }
}