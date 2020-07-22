use clap::Shell;

include!("src/cli.rs");

fn main() {
    let outdir = "completions"; // match env::var_os("OUT_DIR") {
                                //     None => return,
                                //     Some(outdir) => outdir,
                                // };
    let mut app = build_cli();
    app.gen_completions("lamp", Shell::Fish, outdir);
    // app.gen_completions("lamp", Shell::Zsh, outdir);  // TODO search for bug
    app.gen_completions("lamp", Shell::Bash, outdir);
}
