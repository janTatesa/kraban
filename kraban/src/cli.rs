use clap::{
    ArgAction, Parser,
    builder::{Styles, styling::AnsiColor::*},
    command
};

const STYLE: Styles = Styles::styled()
    .header(Green.on_default().bold())
    .usage(Green.on_default().bold())
    .literal(Blue.on_default().bold())
    .placeholder(Cyan.on_default());

#[derive(Parser)]
#[command(styles = STYLE)]
pub struct Cli {
    #[arg(long, short, action = ArgAction::SetTrue, exclusive = true)]
    pub print_default_config: bool,
    #[arg(long, short, action = ArgAction::SetTrue, exclusive = true)]
    pub write_defaul_config: bool
}
