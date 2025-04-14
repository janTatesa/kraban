use clap::{
    Arg, ArgAction, ArgMatches,
    builder::{Styles, styling::AnsiColor::*},
    command,
};
const STYLE: Styles = Styles::styled()
    .header(Green.on_default().bold())
    .usage(Green.on_default().bold())
    .literal(Blue.on_default().bold())
    .placeholder(Cyan.on_default());
const TESTING_HELP: &str = "Use ./testing-files/ instead of normal files for state and config. Useful when you don't want to conflict with your existing kraban installation";
pub fn cli() -> ArgMatches {
    command!()
        .arg(
            Arg::new("testing")
                .long("testing")
                .help(TESTING_HELP)
                .action(ArgAction::SetTrue)
                .env("KRABAN_TESTING"),
        )
        .arg(
            Arg::new("print_default_config")
                .action(ArgAction::SetTrue)
                .short('p')
                .help("Print default config")
                .conflicts_with("write_default_config"),
        )
        .arg(
            Arg::new("write_default_config")
                .action(ArgAction::SetTrue)
                .short('w')
                .help("Write default config to the config dir"),
        )
        .styles(STYLE)
        .get_matches()
}
