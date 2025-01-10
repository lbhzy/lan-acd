use anstyle::AnsiColor;
use clap::builder::styling::Styles;
use clap::Parser;

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::BrightGreen.on_default().bold())
    .usage(AnsiColor::BrightGreen.on_default().bold())
    .literal(AnsiColor::BrightCyan.on_default())
    .placeholder(AnsiColor::BrightBlue.on_default());

#[derive(Parser, Debug)]
#[command(about, version)]
#[command(styles = STYLES)]
pub struct Cli {
    /// List all interfaces and index
    #[arg(short, long)]
    pub list: bool,

    /// Select interface index
    #[arg(short, long, required_unless_present = "list")]
    pub iface: Option<usize>,

    /// Stop if no ARP reply is received after this timeout (ms)
    #[arg(short, long, default_value = "300")]
    pub timeout: u64,
}
