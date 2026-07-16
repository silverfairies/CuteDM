use std::{env, io::Error, path::PathBuf, process::Command};

use cursive::CursiveExt;

use crate::mapper::Choice;

mod mapper;
mod ui;

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

    let tree = Choice::read_tree(PathBuf::from("./examples/tree").read_dir()?)?;

    /*println!(
        "{:#?}",
        &tree
    );*/

    let mut ui = ui::construct_ui(tree);
    ui.run();

    let tree = ui.take_user_data::<Vec<Choice>>().unwrap_or_default();
    println!("{:#?}", &tree);
    let _ = execute_scripts(tree);
    Ok(())
}

fn execute_scripts ( sequence: Vec<Choice> ) -> Result<(), Error> {
    for script in sequence {
        let _ = Command::new(script.path.canonicalize()?.to_str().unwrap_or("echo Some error occured!"))
            .spawn();
            //.args([format!("\"{}\"", script.path.canonicalize()?.to_str().unwrap_or("echo Some error occured!")).as_str()])
    }
    Ok(())
}

//struct Environment {
//    output: String,
//    global_layout:
