#![allow(dead_code)]
use std::sync::{Arc, Mutex};
use std::vec;

use axum::{extract::Query, routing::get, Router};
use config::{ACCIDENT_BEGIN, ACCIDENT_END, H_RANGE_CHANGING_TIME, NUM_CPUS};
use modules::injection::Reactor;
use modules::spreading::Cloud;
use serde::Deserialize;
use services::closures_constructor::construct_injection_closure;
use services::task_broker::{distribute_tasks_by_threads, run_into_threads};

pub mod config;
pub mod modules;
mod services;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/calculate", get(bar));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8888").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct CalcParams {
    latitude: f32,
    longitude: f32,
    productivity: u32,
}

async fn bar(Query(params): Query<CalcParams>) -> String {
    todo!()
}

fn calculate_main(Query(params): Query<CalcParams>) {
    let timer = std::time::Instant::now();
    let reactor = Arc::new(Reactor::new(
        params.latitude,
        params.longitude,
        params.productivity,
    ));
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
