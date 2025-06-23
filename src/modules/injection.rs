#![allow(dead_code)]
use super::spreading::{Cloud, Polygon};

/// Объект реактора (генератора частиц).
/// Генерирует частицы диаметра D на высоте H над собой.
/// H выбирается рандомно в диапазоне 100 - 1000 м. в зависимости от количества дней, прошедших с
/// начала события.
/// D выбирается рандомно в диапазоне mu = 50 мкм, sigma = 0.25
pub struct Reactor {
    pub latitude: f32,  // широта
    pub longitude: f32, // долгота
    productivity: u64,  // производительность выброса [частиц / час]
}

impl Reactor {
    pub fn new(latitude: f32, longitude: f32, productivity: u64) -> Self {
        Reactor {
            latitude,
            longitude,
            productivity,
        }
    }

    pub fn inject(&self, cloud: &mut Cloud) {
        for _ in 0..self.productivity {
            let lat = 1.0;
            let long = 2.0;
            let h = 3.0;
            let d = log_normal.sample(&mut rand::rng());
            println!("d: {d}");
            cloud.add(Polygon::new(lat, long, h, d));
        }
    }
}
