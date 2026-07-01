#![allow(unused)]
use cursive::{Cursive, CursiveExt, theme::{BaseColor, BorderStyle, Color, Palette, PaletteColor, Theme}, views::{self, LinearLayout, Panel, TextView}};


fn main() {
    let mut ui = Cursive::new();

    let mut theme = Theme{ shadow : false, borders : BorderStyle::Simple, palette : Palette::retro()};
    theme.palette[PaletteColor::Background] = Color::Dark(BaseColor::Black);
    theme.palette[PaletteColor::Primary] = Color::Dark(BaseColor::White);
    theme.palette[PaletteColor::View] = Color::Dark(BaseColor::Black);
    ui.set_theme(theme);

    //ui.add_layer( TextView::new("Meow, mrrp!"));
    //ui.add_layer(Panel::new(TextView::new("Mrryaoo!!")));
    ui.add_layer(
        Panel::new(
            LinearLayout::vertical()
                .child(TextView::new("Meowrr"))
                .child(Panel::new(TextView::new(":3")))
        )
    );
    ui.add_global_callback('q', |ui| ui.quit());

    ui.run();
}
