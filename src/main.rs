use std::sync::{Arc, Mutex};
use std::vec;

use config::{ACCIDENT_BEGIN, ACCIDENT_END, H_RANGE_CHANGING_TIME, NUM_CPUS};
use modules::injection::Reactor;
use modules::spreading::Cloud;
use services::task_broker::{break_tasks_by_cores, send_tasks_to_threads};

pub mod config;
pub mod modules;
mod services;

fn main() {
    let timer = std::time::Instant::now();
    let reactor = Arc::new(Reactor::new(52.091943, 47.951047, 1_000_00));
    let cloud = Arc::new(Mutex::new(Cloud::new()));

    let accident_duration = (ACCIDENT_END - ACCIDENT_BEGIN).num_hours() as u16;
    let mut initial_task = vec![];
    for i in 0..accident_duration {
        initial_task.push(i);
    }

    let tasks = break_tasks_by_cores(initial_task, NUM_CPUS);

    let handles = send_tasks_to_threads(tasks, &cloud, &reactor, H_RANGE_CHANGING_TIME);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", timer.elapsed());

    println!("{:?}", cloud.lock().unwrap().get_size())
}
