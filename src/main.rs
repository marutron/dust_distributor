use services::parsers::get_datetime;
use std::sync::{Arc, Mutex};
use std::vec;

use axum::{extract::Query, routing::get, Router};
use config::{H_RANGE_CHANGING_TIME, NUM_CPUS};
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
    let app = Router::new().route("/calculate", get(main_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8888").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct CalcParams {
    latitude: f32,
    longitude: f32,
    productivity: u32,
    acc_begin: String,
    acc_end: String,
}
// 0.0.0.0:8888/calculate?latitude=51.389409&longitude=30.100186&productivity=10000&accident_begin=1986-04-26T01:23:47+0300&accident_end=1986-05-07T18:00:00+0300

async fn main_handler(Query(params): Query<CalcParams>) -> String {
    let accident_begin = match get_datetime(&params.acc_begin) {
        Ok(datetime) => datetime,
        Err(str) => return str,
    };
    let accident_end = match get_datetime(&params.acc_end) {
        Ok(datetime) => datetime,
        Err(str) => return str,
    };

    let acc_duration = (accident_end - accident_begin).num_hours() as u16;
    let timer = std::time::Instant::now();
    let res = calculate_main(
        params.latitude,
        params.longitude,
        params.productivity,
        acc_duration,
    )
    .await;
    println!("{:?}", timer.elapsed());
    res
}

async fn calculate_main(
    latitude: f32,
    longitude: f32,
    productivity: u32,
    accident_duration: u16,
) -> String {
    let reactor = Arc::new(Reactor::new(latitude, longitude, productivity));
    let cloud = Arc::new(Mutex::new(Cloud::new()));

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

    format!("{:?}", cloud.lock().unwrap().get_size())
}
