use async_playground::{
    lifetimes::{arc_runner, owned_runner, ref_runner},
    send_sync::implicit_type::foo,
};
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
        _ if args.module == "lf-arc" => arc_runner().await,
        _ if args.module == "lf-owned" => owned_runner().await,
        _ if args.module == "lf-ref" => ref_runner().await,
        _ => panic!("no matching module"),
    }
}
