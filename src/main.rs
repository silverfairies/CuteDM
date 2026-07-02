#![allow(unused)]
use cursive::{Cursive, CursiveExt, View, direction::Direction, theme::{BaseColor, BorderStyle, Color, Palette, PaletteColor, Theme}, view::{Margins, Nameable}, views::{self, HideableView, LinearLayout, ListView, PaddedView, Panel, SelectView, TextView}};

fn main() {
    let mut ui = Cursive::new();

    let mut theme = Theme{ shadow : false, borders : BorderStyle::Outset, palette : Palette::terminal_default()};
    theme.palette[PaletteColor::Background] = Color::Dark(BaseColor::Black);
    theme.palette[PaletteColor::Primary] = Color::Dark(BaseColor::White);
    theme.palette[PaletteColor::View] = Color::Dark(BaseColor::Black);
    ui.set_theme(theme);

    //ui.add_layer( TextView::new("Meow, mrrp!"));
    //ui.add_layer(Panel::new(TextView::new("Mrryaoo!!")));
    let mut list = SelectView::new()
        .item("Mrrp", "Mrrp")
        .item("Nya", "Nya")
        .item("Meow", "Meow")

        .on_submit(|ui, choice: &str| {
            ui.set_user_data(choice.to_string());
            
            ui.call_on_name("menu", |menu_wrapper: &mut HideableView<PaddedView<SelectView<&str>>>| {
                menu_wrapper.unhide();
                //menu_wrapper.get_inner_mut().enable();
            });
            ui.focus_name("menu");
        }
    );

    let list2 = SelectView::new()
        .item("blank", "")
        .item("period", ".")
        .item("exclamation mark", "!")
        .item("colon three", ":3")

        .on_submit(|ui, choice: &str| {
            let mut output = ui.take_user_data().unwrap_or("error!").to_string();
            output.push_str(choice);
            ui.set_user_data(output);
            ui.quit();
        }
    ).with_name("menu0");

    let list2wrap = HideableView::new(PaddedView::new(Margins::tb(1, 0), list2)).hidden().with_name("menu");

    ui.add_layer(
        Panel::new(
            LinearLayout::vertical()
                .child(TextView::new("Meowrr"))
                .child(Panel::new(TextView::new(":3")))
                .child(list)
                .child(list2wrap)
        ).title("Meower")
    );
    ui.add_global_callback('q', |ui| ui.quit());

    ui.run();

    println!("{}", ui.user_data().unwrap_or(&mut "error ocured".to_string()))
}

//struct Environment {
//    output: String,
//    global_layout: 
