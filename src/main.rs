use async_playground::lifetimes::{arc_run, owned_runner};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of module to run
    #[arg(short, long)]
    module: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args.module {
        _ if args.module == "lf-arc" => arc_run().await,
        _ if args.module == "lf-owned" => owned_runner().await,
        _ => panic!("no matching module"),
    }
}
