use chrono::Duration;
use config::{ACCIDENT_BEGIN, ACCIDENT_END};
use modules::injection::Reactor;
use modules::spreading::Cloud;

pub mod config;
mod modules;

fn main() {
    let timer = std::time::Instant::now();
    let reactor = Reactor::new(52.091943, 47.951047, 1_000_00);
    let mut cloud = Cloud::new();

    let accident_duration = (ACCIDENT_END - ACCIDENT_BEGIN).num_hours();
    for hour in 0..accident_duration {
        let injected = reactor.inject(hour);
        cloud.extend(injected)
    }
    println!("{:?}", timer.elapsed());

    println!("{:?}", cloud.get_size())
}
