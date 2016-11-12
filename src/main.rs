extern crate futures;
extern crate futures_cpupool;
extern crate tokio_core;

use futures::*;
use futures::stream::Stream;
use std::io::prelude::*;
use tokio_core::reactor::Core;

fn main() {
    let stdin = std::io::stdin();
    let lock = stdin.lock();

    let lines = stream::iter(lock.lines());

    let mut pool_builder = futures_cpupool::Builder::new();
    let pool = pool_builder.create();

    let workers = lines.map(|l| {
        let future = finished::<String, ()>(l)
            .map(|l| l.to_uppercase())
            .and_then(|l| {
                println!("{}", l);
                finished(())
            });

        pool.spawn(future)
    });

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let server = workers.for_each(|future| {
        handle.spawn(future);
        Ok(())
    });

    core.run(server).unwrap();
}
