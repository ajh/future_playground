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

    if false {
        with_thread_pool(lines);
    }
    else {
        without_thread_pool(lines);
    }
}

fn with_thread_pool(lines: futures::stream::Select<futures::stream::IterStream<std::io::Lines<std::io::StdinLock>>, futures::stream::IterStream<std::io::Lines<std::io::BufReader<std::fs::File>>>>) {
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

fn without_thread_pool(lines: futures::stream::Select<futures::stream::IterStream<std::io::Lines<std::io::StdinLock>>, futures::stream::IterStream<std::io::Lines<std::io::BufReader<std::fs::File>>>>) {
    // create a tokio reactor
    let mut core = Core::new().unwrap();

    // run all the stuff
    core.run(lines.for_each(|l| {
        println!("{}", l.to_uppercase());
        Ok(())
    })).unwrap();
}
