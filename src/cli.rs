use anstyle::AnsiColor;
use clap::builder::styling::Styles;
use clap::Parser;

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::BrightGreen.on_default().bold())
    .usage(AnsiColor::BrightGreen.on_default().bold())
    .literal(AnsiColor::BrightCyan.on_default())
    .placeholder(AnsiColor::BrightBlue.on_default());

#[derive(Parser, Debug)]
#[command(about = format!("{} ({})", env!("CARGO_PKG_DESCRIPTION"), env!("BUILD_DATE")))]
#[command(styles = STYLES)]
pub struct Cli {
    /// List all interfaces and index
    #[arg(short, long)]
    pub list: bool,

    /// Select interface index
    #[arg(short, long, required_unless_present = "list")]
    pub iface: Option<usize>,

    /// Stop if no ARP reply beyond this time (ms)
    #[arg(short, long, default_value = "300")]
    pub timeout: u64,
}
