use clap::{App, AppSettings, Arg};

pub fn app() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("cmd")
                .help("Command to send to the delay server.")
                .index(1)
                .required(false)
                .takes_value(true)
                .value_name("CMD"),
        )
        .arg(
            Arg::with_name("delay")
                .help("Milliseconds to delay the command by (max 9999)")
                .required(false)
                .takes_value(true)
                .value_name("DELAY")
                .default_value("500")
                .long("delay")
                .short("d"),
        )
        .arg(
            Arg::with_name("server")
                .help("run this utility in server mode")
                .required(false)
                .long("server"),
        )
}
