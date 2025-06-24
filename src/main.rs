use modules::injection::Reactor;
use modules::spreading::Cloud;

mod modules;

fn main() {
    let reactor = Reactor::new(52.091943, 47.951047, 1_000_000);
    let mut cloud = Cloud::new();

    reactor.inject(&mut cloud, 4);
    println!("{:?}", cloud.get_size())
}
