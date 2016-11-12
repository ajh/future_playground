extern crate futures;
extern crate tokio_core;

use futures::*;
use futures::stream::Stream;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().unwrap();

    // open file
    let mut f = File::open("README.md").unwrap();
    let reader = BufReader::new(f);

    // convert to stream
    let lines = stream::iter(reader.lines());

    // for lines, upcase them and print them
    let print_upcased_lines = lines.and_then(|line| {
        println!("{}", line.to_uppercase());
    });

    // start core
    core.run(print_upcased_lines).unwrap();
}
