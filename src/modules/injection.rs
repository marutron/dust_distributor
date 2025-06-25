#![allow(dead_code)]

use super::super::config::{H_RANGE_CHANGING_TIME, LOWER_H_RANGE, SIZE_RANGE, UPPER_H_RANGE};
use super::spreading::{Cloud, Polygon};
use rand::Rng;

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

    pub fn inject(&self, cloud: &mut Cloud, hours: u16) {
        let mut rng = rand::rng();
        let h_range = if hours <= H_RANGE_CHANGING_TIME {
            LOWER_H_RANGE
        } else {
            UPPER_H_RANGE
        };
        for _ in 0..self.productivity {
            let lat = self.latitude;
            let long = self.longitude;
            let h = rng.random_range(h_range.0..h_range.1);
            let d: f32 = rng.random_range(SIZE_RANGE.0..SIZE_RANGE.1);
            cloud.add(Polygon::new(lat, long, h, d));
        }
    }
}
