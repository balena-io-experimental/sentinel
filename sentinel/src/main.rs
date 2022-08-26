// Take a look at the license at the top of the repository in the LICENSE file.

#![crate_type = "bin"]
#![allow(unused_must_use, non_upper_case_globals)]

extern crate sysinfo;

use sysinfo::{
    System, SystemExt,
};
use clap::Parser;
use anyhow::{Result};
// use log::{info, warn};

#[derive(Parser)]
struct Cli {
    /// The information to look for
    info: String,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Cli::parse();
    println!("Getting processes' information...");
    let mut t = System::new_all();
    println!("Done.");

    sentinel::system_info(&args.info, &mut t, &mut std::io::stdout());

    Ok(())
}
