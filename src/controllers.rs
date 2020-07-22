use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::exit;

use exitcode;

const SYS_PATHS: [&str; 2] = ["/sys/class/backlight", "/sys/class/leds"];

pub trait Controller {
    fn get_brightness(&self) -> i32;
    fn get_max_brightness(&self) -> i32;
    fn set_brightness(&self, value: i32);

    fn check_brightness_value(&self, value: i32) {
        if value > self.get_max_brightness() {
            eprintln!(
                "brightness value too high: {} > {}",
                value,
                self.get_max_brightness()
            );
            exit(exitcode::DATAERR);
        } else if value < 0 {
            eprintln!("brightness value too low: {}", value);
            exit(exitcode::DATAERR);
        }
    }
}

pub struct RawController {
    path: Box<PathBuf>,
}

impl RawController {
    pub fn new(path: Box<PathBuf>) -> Self {
        Self { path: path }
    }
}

impl Controller for RawController {
    fn get_brightness(&self) -> i32 {
        read_file_to_int(self.path.join("brightness"))
    }

    fn get_max_brightness(&self) -> i32 {
        read_file_to_int(self.path.join("max_brightness"))
    }

    fn set_brightness(&self, value: i32) {
        self.check_brightness_value(value);

        let path = self.path.join("brightness");

        let mut file = match OpenOptions::new().write(true).read(true).open(&path) {
            Err(why) => {
                eprintln!("couldn't open '{}': {:?}", &path.display(), why.kind());
                exit(exitcode::OSFILE);
            }
            Ok(file) => file,
        };

        match write!(file, "{}", value) {
            Ok(_) => {}
            Err(err) => {
                eprintln!(
                    "could not write '{}' to file '{}': {:?}",
                    value,
                    &path.display(),
                    err.kind()
                );
                exit(exitcode::OSFILE);
            }
        };
    }
}

pub struct LinController {
    parent_controller: RawController,
}

impl LinController {
    pub fn new(path: Box<PathBuf>) -> Self {
        Self {
            parent_controller: RawController::new(path),
        }
    }
}

impl Controller for LinController {
    fn get_brightness(&self) -> i32 {
        ((self.parent_controller.get_brightness() as f64
            / self.parent_controller.get_max_brightness() as f64)
            * self.get_max_brightness() as f64) as i32
    }

    fn get_max_brightness(&self) -> i32 {
        100
    }

    fn set_brightness(&self, value: i32) {
        self.check_brightness_value(value);

        if value > self.get_max_brightness() {
            eprintln!(
                "brightness value too high! {} > {}",
                value,
                self.get_max_brightness()
            );
            exit(exitcode::DATAERR);
        }

        self.parent_controller.set_brightness(
            (value * self.parent_controller.get_max_brightness()) / self.get_max_brightness(),
        )
    }
}

pub struct LogController {
    parent_controller: RawController,
}

impl LogController {
    pub fn new(path: Box<PathBuf>) -> Self {
        Self {
            parent_controller: RawController::new(path),
        }
    }
}

impl Controller for LogController {
    fn get_brightness(&self) -> i32 {
        ((self.parent_controller.get_brightness() as f64).log10()
            / (self.parent_controller.get_max_brightness() as f64).log10()
            * self.get_max_brightness() as f64) as i32
    }

    fn get_max_brightness(&self) -> i32 {
        100
    }

    fn set_brightness(&self, value: i32) {
        self.check_brightness_value(value);

        if value > self.get_max_brightness() {
            eprintln!(
                "brightness value too high! {} > {}",
                value,
                self.get_max_brightness()
            );
            exit(exitcode::DATAERR);
        }

        self.parent_controller.set_brightness(10f64.powf(
            (value as f64 / self.get_max_brightness() as f64)
                * (self.parent_controller.get_max_brightness() as f64).log10(),
        ) as i32)
    }
}

fn read_file_to_int(path: PathBuf) -> i32 {
    let mut file = match File::open(&path) {
        Err(why) => {
            eprintln!("couldn't open {}: {:?}", path.display(), why.kind());
            exit(exitcode::OSFILE);
        }
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => {
            eprintln!("couldn't read {}: {:?}", path.display(), why.kind());
            exit(exitcode::OSFILE);
        }
        Ok(_) => return s.trim().parse().unwrap(),
    }
}

/// Searches through all paths in `SYS_PATHS` and creates a `HashMap` with the name and absolute path.
///
/// It returns a `Tuple` of the default backlight name and the `HashMap`.
pub fn get_controllers() -> (String, HashMap<String, Box<PathBuf>>) {
    let mut controllers: HashMap<String, Box<PathBuf>> = HashMap::new();

    let mut default = None;

    for path in SYS_PATHS.iter() {
        if Path::new(path).exists() {
            for name in Path::new(path).read_dir().unwrap() {
                let name = name.unwrap().path();
                let key = String::from(name.file_name().unwrap().to_str().unwrap());

                if default.is_none() {
                    default = Some(key.clone());
                }

                controllers.insert(key, Box::new(name));
            }
        }
    }

    (default.unwrap(), controllers)
}
