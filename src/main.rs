use std::{io::Error, path::PathBuf, process::Command};

use cursive::{
    Cursive, CursiveExt,
    theme::{BaseColor, BorderStyle, Color, Palette, PaletteColor, Theme},
    view::{Margins, Nameable},
    views::{self, HideableView, LinearLayout, PaddedView, Panel, SelectView, TextView},
};

use crate::mapper::Choice;

mod mapper;

fn main() -> Result<(), Error> {
    /*
    let mut ui = Cursive::new();

    ui.set_theme(theme_constructor());

    let list = SelectView::new()
        .item("Mrrp", "Mrrp")
        .item("Nya", "Nya")
        .item("Meow", "Meow")
        .on_submit(|ui, choice: &str| {
            ui.set_user_data(choice.to_string());
            ui.call_on_name("menu0", |menu_zero: &mut SelectView<&str>| {
                menu_zero.disable()
            });
            ui.call_on_name(
                "menu",
                |menu_wrapper: &mut HideableView<PaddedView<SelectView<&str>>>| {
                    menu_wrapper.unhide();
                    //menu_wrapper.get_inner_mut().enable();
                },
            );
            let _ = ui.focus_name("menu");
        })
        .with_name("menu0");

    let list2 = SelectView::new()
        .item("blank", "")
        .item("period", ".")
        .item("exclamation mark", "!")
        .item("colon three", ":3")
        .on_submit(|ui, choice: &str| {
            let mut output = ui
                .take_user_data()
                .unwrap_or("error!".to_string())
                .to_string();
            output.push_str(choice);
            ui.set_user_data(output);
            ui.quit();
        });

    let list2wrap = HideableView::new(PaddedView::new(Margins::tb(1, 0), list2))
        .hidden()
        .with_name("menu");

    ui.add_layer(
        Panel::new(
            LinearLayout::vertical()
                .child(TextView::new("Meowrr"))
                .child(Panel::new(TextView::new(":3")))
                .child(list)
                .child(list2wrap),
        )
        .title("Meower"),
    );
    ui.add_global_callback('q', |ui| ui.quit());

    ui.run();

    let mut problem = "error occured".to_string();

    let input = ui.user_data().unwrap_or(&mut problem);
    //println!(
    //    "{}",
    //    &input
    //);

    let _ = Command::new("sh")
        .args(["-c", format!("echo {}", input).as_str()])
        .spawn();
    */

    println!(
        "{:#?}",
        Choice::read_tree(
            PathBuf::from("/home/argentum/Documents/Projects/Rust/cutedm/tmp").read_dir()?
        )?
    );
    Ok(())
}

#[allow(unused)]
fn theme_constructor() -> Theme {
    let mut theme = Theme {
        shadow: false,
        borders: BorderStyle::Outset,
        palette: Palette::terminal_default(),
    };
    theme.palette[PaletteColor::Background] = Color::Dark(BaseColor::Black);
    theme.palette[PaletteColor::Primary] = Color::Dark(BaseColor::White);
    theme.palette[PaletteColor::View] = Color::Dark(BaseColor::Black);
    theme
}

//struct Environment {
//    output: String,
//    global_layout:
