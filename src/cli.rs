use clap::Parser;
use anstyle::AnsiColor;
use clap::builder::styling::Styles;

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default())
    .usage(AnsiColor::Green.on_default())
    .literal(AnsiColor::Cyan.on_default())
    .placeholder(AnsiColor::Red.on_default());

#[derive(Parser, Debug)]
#[command(about = "LAN address conflict detection")]
#[command(styles = STYLES)]
pub struct Cli {
    /// List all interfaces and index
    #[arg(short, long)]
    pub list: bool,

    /// Select interface index
    #[arg(short, long, required_unless_present = "list")]
    pub iface: Option<usize>,

    /// Scanning ends if no ARP response is received beyond this time (ms)
    #[arg(short, long, default_value = "1000")]
    pub timeout: u64,
}