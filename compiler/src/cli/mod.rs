use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(about, version, author)]
pub struct Args {
    /// Input file
    #[clap(short, long)]
    pub input_file: String,

    /// Output file
    #[clap(short, long)]
    pub output_file: Option<String>,

    /// Enable verbose output
    #[clap(short, long)]
    pub verbose: bool,

    /// Apply optimizations
    #[clap(long)]
    pub release: bool,

    /// Run only translation phase, not compilation
    #[clap(long)]
    pub dry_run: bool,
}
