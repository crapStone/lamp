use clap::{App, Arg, ArgGroup, ArgMatches};

pub fn build_cli() -> App<'static, 'static> {
    App::new("RS Light")
        .version("1.0")
        .author("crapStone <wewr.mc@gmail.com>")
        .about("Utility to interact with backlight")
        .arg(
            Arg::with_name("set")
                .short("s")
                .long("set")
                .value_name("VALUE")
                .help("Sets brightness to given value")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("inc")
                .short("i")
                .long("increase")
                .value_name("PERCENT")
                .help("Increases brightness")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dec")
                .short("d")
                .long("decrease")
                .value_name("PERCENT")
                .help("Decreases brightness")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("get")
                .short("g")
                .long("get")
                .help("Prints current brightness value"),
        )
        .arg(
            Arg::with_name("zer")
                .short("z")
                .long("zero")
                .help("Sets brightness to lowest value"),
        )
        .arg(
            Arg::with_name("ful")
                .short("f")
                .long("full")
                .help("Sets brightness to highest value"),
        )
        .group(ArgGroup::with_name("brightness_control").args(&["set", "inc", "dec", "get", "zer", "ful"]))
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("Lists all available brightness and led controllers")
                .conflicts_with_all(&["brightness_control"]),
        )
        .arg(
            Arg::with_name("ctrl_type")
                .short("t")
                .long("type")
                .value_name("controller_type")
                .takes_value(true)
                .possible_values(&["raw", "lin", "log"])
                .default_value("lin")
                .help("choose controller type")
                .long_help(
                    r#"You can choose between these controller types:
raw: uses the raw values found in the device files
lin: uses percentage values (0.0 - 1.0) with a linear curve for the actual brightness
log: uses percentage values (0.0 - 1.0) with a logarithmic curve for the actual brightness
     the perceived brightness for the human eyes should be linear with this controller
"#,
                ),
        )
}

/// Creates a argument parser with [clap](../clap/index.html) and returns a `Box` with the
/// [matches](../clap/struct.ArgMatches.html).
pub fn parse_args<'a>() -> Box<ArgMatches<'a>> {
    Box::new(build_cli().get_matches())
}
