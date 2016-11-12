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
    let f = File::open("README.md").unwrap();
    let reader = BufReader::new(f);

    // convert to stream
    // reader.lines() is std::io::Lines<std::io::BufReader<std::fs::File>>
    let lines = stream::iter(reader.lines());
    // lines is futures::stream::IterStream<std::io::Lines<std::io::BufReader<std::fs::File>>>

    // for lines, upcase them and print them
    let print_upcased_lines = lines.and_then(|line| {
        println!("{}", line.to_uppercase());
        finished(())
    });

    let future = print_upcased_lines.for_each(|_| Ok(()));

    // start core
    core.run(future).unwrap();
}
