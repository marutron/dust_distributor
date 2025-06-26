use std::sync::{Arc, Mutex};
use std::{thread, vec};

use config::{ACCIDENT_BEGIN, ACCIDENT_END, NUM_CPUS};
use modules::injection::Reactor;
use modules::spreading::Cloud;

pub mod config;
mod modules;

fn main() {
    let timer = std::time::Instant::now();
    let reactor = Arc::new(Reactor::new(52.091943, 47.951047, 1_000_00));
    let cloud = Arc::new(Mutex::new(Cloud::new()));

    let accident_duration = (ACCIDENT_END - ACCIDENT_BEGIN).num_hours() as u16;
    let task_count = accident_duration / NUM_CPUS;
    let mut handles = vec![];

    for i in 0..NUM_CPUS {
        let cloud = Arc::clone(&cloud);
        let reactor = Arc::clone(&reactor);
        let handle = thread::spawn(move || {
            for hour in 0..task_count {
                let res = reactor.inject(hour);
                let mut cloud_mut = cloud.lock().unwrap();
                cloud_mut.extend(res);
            }
            let cloud = cloud.lock().unwrap();
            println!("pid: {i}, cloud size: {:?}", cloud.get_size())
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", timer.elapsed());

    println!("{:?}", cloud.lock().unwrap().get_size())
}
