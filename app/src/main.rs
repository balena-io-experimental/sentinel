// Take a look at the license at the top of the repository in the LICENSE file.

#![crate_type = "bin"]
#![allow(unused_must_use, non_upper_case_globals)]

extern crate sysinfo;

use sysinfo::{
    System, SystemExt,
};
use clap::Parser;
use anyhow::Result;
use log::{info, warn};
use std::env; // get list of checks as an environemtn variable
use std::{thread, time};

#[derive(Parser)]
struct Cli {
    /// The information to look for
    info: String,
}

fn main() -> Result<()> {
    env_logger::init();
    // let args = Cli::parse();
    loop {
        info!("Getting processes' information...");
        let mut t = System::new_all();
        warn!("Done.");

        // match env::var_os("CHECKS") { 
        //     Ok(v) => {
        //         println!("{}: {}", checks, v);
        //         sentinel::system_info(checks, &mut t, &mut std::io::stdout());
        //     }
        //     Err(e) => {
        //         println!("${} is not set ({})", checks, e);
        //         sentinel::system_info(&args.info, &mut t, &mut std::io::stdout());
        //     }
        /* } */
        sentinel::check_os();

        let checks_env = match env::var_os("CHECKS") {
            Some(v) => v.into_string().unwrap(),
            None => {
                panic!("$CHECKS is not set");
            }
        };
        let checks = checks_env.split(' ');
        for check in checks {
            println!("{}", check);
            sentinel::system_info(check, &mut t, &mut std::io::stdout());
        }

        let delay_env = match env::var_os("DELAY_SECONDS") {
            // Some(v) => v.into_string().unwrap(),
            Some(v) => v.into_string().unwrap().trim().parse().expect("Wanted a number"),
            None => {
                panic!("$DELAY_SECONDS is not set");
            }
        };
        println!("delaying for {} seconds", delay_env);

        let delay_seconds = time::Duration::from_secs(delay_env);
        let now = time::Instant::now();

        thread::sleep(delay_seconds);

        assert!(now.elapsed() >= delay_seconds);
    }
    //
    // Ok(());
}
