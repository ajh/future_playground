extern crate futures;
extern crate futures_cpupool;
extern crate tokio_core;

use futures::*;
use std::io::prelude::*;
use tokio_core::reactor::Core;
use std::fs::File;
use std::io::BufReader;

fn main() {
    // get `lines`, a Stream over stdin lines
    let stdin = std::io::stdin();
    let lock = stdin.lock();
    let lines = stream::iter(lock.lines());

    let readme = BufReader::new(File::open("README.md").unwrap());
    let lines = lines.select(stream::iter(readme.lines()));

    // create cpu pool for running futures
    let mut pool_builder = futures_cpupool::Builder::new();
    let pool = pool_builder.create();

    // for each line, run transforms in cpu pool. `workers` is a stream of futures returned by cpu
    // pool.
    let workers = lines.map(|l| {
        let future = finished::<String, ()>(l)
            .map(|l| l.to_uppercase())
            .and_then(|l| {
                println!("{}", l);
                finished(())
            });

        pool.spawn(future)
    });

    // create a tokio reactor
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    // add cpu pool futures to reactor
    let server = workers.for_each(|future| {
        handle.spawn(future);
        Ok(())
    });

    // run all the stuff
    core.run(server).unwrap();
}
