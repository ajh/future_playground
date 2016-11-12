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

    let f = File::open("README.md").unwrap();
    let reader = BufReader::new(f);

    let lines = stream::iter(reader.lines());

    let uplines = lines.map(|l| {
        println!("{}", l.to_uppercase());
        finished(())
    });

    let handle = core.handle();
    let server = uplines.for_each(|future| {
        handle.spawn(future);
        Ok(())
    });

    // start core
    core.run(server).unwrap();
}
