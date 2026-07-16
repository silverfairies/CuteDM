use std::{env, io::Error, path::PathBuf, process::Command};

use cursive::CursiveExt;

use crate::mapper::Choice;

mod mapper;
mod ui;

fn main() -> Result<(), Error> {
    let mut args = env::args();
    let tree = args.nth(1);
    if tree.as_ref().is_none_or(|path| !PathBuf::from(path).is_dir()) {
        println!("Specify a valid directory!");
    } else {
        let tree = Choice::read_tree(PathBuf::from(tree.unwrap()).read_dir()?)?;
        let mut ui = ui::construct_ui(tree);
        ui.run();

        let tree = ui.take_user_data::<Vec<Choice>>().unwrap_or_default();
        //println!("{:#?}", &tree);
        let _ = execute_scripts(tree);
    }
    Ok(())
}

fn execute_scripts(sequence: Vec<Choice>) -> Result<(), Error> {
    for script in sequence {
        let _ = Command::new(
            script
                .path
                .canonicalize()?
                .to_str()
                .unwrap_or("echo Some error occured!"),
        )
        .spawn();
        //.args([format!("\"{}\"", script.path.canonicalize()?.to_str().unwrap_or("echo Some error occured!")).as_str()])
    }
    Ok(())
}
