use std::sync::{Arc, Mutex};
use std::vec;

use config::{ACCIDENT_BEGIN, ACCIDENT_END, H_RANGE_CHANGING_TIME, NUM_CPUS};
use modules::injection::Reactor;
use modules::spreading::Cloud;
use services::closures_constructor::construct_injection_closure;
use services::task_broker::{distribute_tasks_by_threads, run_into_threads};

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

    let tasks = distribute_tasks_by_threads(initial_task, NUM_CPUS);
    let moved_cloud = Arc::clone(&cloud);
    let moved_reactor = Arc::clone(&reactor);

    let injection_clos =
        construct_injection_closure(moved_cloud, moved_reactor, H_RANGE_CHANGING_TIME);

    let handles = run_into_threads(tasks, move |task| injection_clos(task));

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", timer.elapsed());

    println!("{:?}", cloud.lock().unwrap().get_size())
}
