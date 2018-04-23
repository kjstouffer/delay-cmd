extern crate clap;
use clap::Shell;
use std::env;

#[path = "src/load_config.rs"]
mod load_config;

fn main() {
    let mut app = load_config::app();
    app.gen_completions("delay-cmd", Shell::Zsh, env::var("OUT_DIR").unwrap());
    app.gen_completions("delay-cmd", Shell::Bash, env::var("OUT_DIR").unwrap());
    app.gen_completions("delay-cmd", Shell::Fish, env::var("OUT_DIR").unwrap());
    app.gen_completions(
        "delay-cmd",
        Shell::PowerShell,
        env::var("OUT_DIR").unwrap(),
    );
}
