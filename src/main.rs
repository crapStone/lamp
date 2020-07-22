mod cli;
mod controllers;

use std::process::exit;

use exitcode;

use controllers::{Controller, LinController, LogController, RawController};

fn main() {
    let matches = cli::parse_args();

    let (default_ctrl, ctrls) = controllers::get_controllers();

    let p = ctrls.get(&default_ctrl).unwrap().to_owned();
    let controller: Box<dyn Controller> = match matches.value_of("ctrl_type") {
        Some("raw") => Box::new(RawController::new(p)),
        Some("lin") => Box::new(LinController::new(p)),
        Some("log") => Box::new(LogController::new(p)),
        Some(_) | None => panic!(ERROR_MSG),
    };

    if matches.is_present("list") {
        for ctrl in ctrls.keys() {
            println!("{}", ctrl);
        }
        exit(exitcode::OK);
    } else if let Some(value) = matches.value_of("set") {
        let new_value = value.parse::<i32>().unwrap();
        controller.set_brightness(new_value);
    } else if let Some(value) = matches.value_of("inc") {
        let new_value = controller.get_brightness() + value.parse::<i32>().unwrap();
        controller.set_brightness(new_value.min(controller.get_max_brightness()));
    } else if let Some(value) = matches.value_of("dec") {
        let new_value = controller.get_brightness() - value.parse::<i32>().unwrap();
        controller.set_brightness(new_value.max(0));
    } else if matches.is_present("get") {
        println!("{}", controller.get_brightness());
    } else if matches.is_present("zer") {
        controller.set_brightness(0);
    } else if matches.is_present("ful") {
        controller.set_brightness(controller.get_max_brightness());
    } else {
        panic!(ERROR_MSG);
    }

    exit(exitcode::OK);
}

// https://xkcd.com/2200/
const ERROR_MSG: &str = r#"
    ERROR!

    If you're seeing this, the code is in what I thought was an unreachable state.

    I could give you advice for what to do. but honestly, why should you trust me?
    I clearly screwed this up. I'm writing a message that should never appear,
    yet I know it will probably appear someday.

    On a deep level, I know I'm not up to this task. I'm so sorry.
"#;
