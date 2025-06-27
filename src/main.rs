use std::sync::{Arc, Mutex};
use std::{thread, vec};

use config::{ACCIDENT_BEGIN, ACCIDENT_END, H_RANGE_CHANGING_TIME, NUM_CPUS};
use modules::injection::Reactor;
use modules::spreading::Cloud;
use services::task_broker::break_tasks_by_cores;

pub mod config;
pub mod modules;
mod services;

fn main() {
    let timer = std::time::Instant::now();
    let reactor = Arc::new(Reactor::new(52.091943, 47.951047, 1_000_00));
    let cloud = Arc::new(Mutex::new(Cloud::new()));

    let accident_duration = (ACCIDENT_END - ACCIDENT_BEGIN).num_hours() as u16;
    let tasks = break_tasks_by_cores(accident_duration, NUM_CPUS);

    let mut handles = vec![];

    for task in tasks {
        let cloud = Arc::clone(&cloud);
        let reactor = Arc::clone(&reactor);
        let handle = thread::spawn(move || {
            for hour in task {
                let res = reactor.inject(hour, H_RANGE_CHANGING_TIME);
                let mut cloud_mut = cloud.lock().unwrap();
                cloud_mut.extend(res);
            }
            let cloud = cloud.lock().unwrap();
            println!("some pid worked. cloud size: {:?}", cloud.get_size())
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", timer.elapsed());

    println!("{:?}", cloud.lock().unwrap().get_size())
}
