extern crate clap;
extern crate md5;
extern crate rayon;
extern crate walkdir;

pub mod core;
pub mod io;

use clap::{Arg, App};
use core::engine::Engine;
use rayon::prelude::*;
use std::path::Path;

const NAME: &'static str = "TrustAntivirus";
const VERSION: &'static str = "1.0.0";
const AUTHOR: &'static str = "Adam Childs <adamdchilds@gmail.com>";
const ABOUT: &'static str = "";

fn main() {
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .arg(Arg::with_name("DIRECTORY")
            .help("Sets the input directory to use")
            .required(true)
            .index(1))
        .get_matches();

    let dir: &str = matches.value_of("DIRECTORY").unwrap_or("/Users/adchilds/Desktop");

    let engine: Engine = Engine::from(dir);
    engine.do_scan();
}