use cursive::{
    Cursive,
    theme::{BaseColor, BorderStyle, Color, Palette, PaletteColor, Theme},
    view::{Nameable, ViewWrapper},
    views::{NamedView, Panel, SelectView},
};

use crate::mapper::Choice;

pub fn construct_ui(tree: Vec<Choice>) -> Cursive {
    let mut ui = Cursive::new();
    ui.set_theme(theme_constructor());
    global_callbacks(&mut ui);
    ui.set_user_data(Vec::<Choice>::new());

    let mut list = SelectView::<Choice>::new();

    for entry in tree {
        list.add_item(entry.name.clone(), entry);
    }

    list.set_on_submit(|ui, choice: &Choice| {
        let mut tmp = ui.take_user_data().unwrap_or(Vec::<Choice>::new());
        tmp.push(choice.clone());
        ui.set_user_data(tmp);
        //ui.set_user_data(choice.clone());
        if choice.subchoices.is_empty() {
            ui.quit();
        } else {
            ui.call_on_name("menu", |menu: &mut NamedView<SelectView<Choice>>| {
                menu.with_view_mut(|list: &mut SelectView<Choice>| {
                    list.clear();
                    for entry in &choice.subchoices {
                        list.add_item(entry.name.clone(), entry.clone());
                    }
                })
            });
        }
    });

    let menu = list.with_name("menu");

    ui.add_layer(Panel::new(menu).title("CuteDM"));

    ui
}

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

fn global_callbacks(ui: &mut Cursive) {
    ui.add_global_callback('q', |ui| ui.quit());
}
