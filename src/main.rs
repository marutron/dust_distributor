use chrono::Duration;
use config::{ACCIDENT_BEGIN, ACCIDENT_END};
use modules::injection::Reactor;
use modules::spreading::Cloud;

pub mod config;
mod modules;

fn main() {
    let reactor = Reactor::new(52.091943, 47.951047, 1_000_000);
    let mut cloud = Cloud::new();

    let time_delta = reactor.inject(&mut cloud, 4);
    println!("{:?}", cloud.get_size())
}
