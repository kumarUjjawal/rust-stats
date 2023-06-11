//use srcstats;
use srcstats::get_summary_src_stats;
use std::path::PathBuf;
use structopt::StructOpt;
mod errors;
use errors::StatsError;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rust-stats",
    about = "This is a tool to generate statistics on Rust Projects"
)]

struct Opt {
    #[structopt(name = "source directory", parse(from_os_str))]
    in_dir: PathBuf,
    #[structopt(name = "mode", short)]
    mode: String,
}

fn main() -> Result<(), StatsError> {
    let opt = Opt::from_args();
    let mode = &opt.mode[..];
    match mode {
        "src" => {
            let stats = get_summary_src_stats(&opt.in_dir)?;
            println!("Summary stats: {:?}", stats);
        }

        _ => print!("Sorry, no stats"),
    }

    Ok(())
}
