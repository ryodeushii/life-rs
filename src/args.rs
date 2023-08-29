use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None, name="life")]
pub struct Args {
    #[arg(short, long = "width", default_value = "25")]
    width: u32,
    #[arg(short, long = "height", default_value = "25")]
    height: u32,
    #[arg(short, long, default_value = "false")]
    random: bool,
}
