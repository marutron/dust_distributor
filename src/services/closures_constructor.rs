use std::sync::{Arc, Mutex};
use std::thread;

use crate::modules::{injection::Reactor, spreading::Cloud};

/// Конструирует объект замыкания Reactor.inject для передачи для выполнения в потоки
pub fn construct_injection_closure(
    cloud: Arc<Mutex<Cloud>>,
    reactor: Arc<Reactor>,
    changing_time: u16,
) -> impl Fn(Vec<u16>) {
    let cloud = Arc::clone(&cloud);
    let reactor = Arc::clone(&reactor);
    move |task| {
        let cloud = Arc::clone(&cloud);
        let reactor = Arc::clone(&reactor);
        for hour in task {
            let res = reactor.inject(hour, changing_time);
            let mut cloud_mut = cloud.lock().unwrap();
            cloud_mut.extend(res);
        }
        let cloud = cloud.lock().unwrap();
        println!(
            "thread {:?} worked. cloud size: {:?}",
            thread::current().id(),
            cloud.get_size()
        );
    }
}
