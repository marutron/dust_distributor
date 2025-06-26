#![allow(dead_code)]

use super::super::config::{H_RANGE_CHANGING_TIME, LOWER_H_RANGE, SIZE_RANGE, UPPER_H_RANGE};
use super::spreading::Polygon;
use rand::Rng;

/// Объект реактора (генератора частиц).
/// Генерирует частицы диаметра D на высоте H над собой.
/// H выбирается рандомно в диапазоне 100 - 1000 м. в зависимости от количества дней, прошедших с
/// начала события.
/// D выбирается рандомно в диапазоне mu = 50 мкм, sigma = 0.25
#[derive(Clone)]
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

    /// Функция формирования выброса.
    /// Испускает Reactor.productivity частиц (полигонов) в единицу времени (обычно в час)
    pub fn inject(&self, hours: u16) -> Vec<Polygon> {
        let mut rng = rand::rng();
        let h_range = if hours <= H_RANGE_CHANGING_TIME {
            LOWER_H_RANGE
        } else {
            UPPER_H_RANGE
        };
        let mut addition = vec![];
        let lat = self.latitude;
        let long = self.longitude;
        for _ in 0..self.productivity {
            let h = rng.random_range(h_range.0..h_range.1);
            let d: f32 = rng.random_range(SIZE_RANGE.0..SIZE_RANGE.1);
            addition.push(Polygon::new(lat, long, h, d));
        }
        addition
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn injection_in_lower_h_range() {
        let reactor = Reactor::new(52.091943, 47.951047, 1_000_000);
        let hour = 1;
        let res = reactor.inject(hour);
        assert_eq!(res.len(), 1_000_000);

        for poly in res {
            assert!(poly.height > LOWER_H_RANGE.0 && poly.height < LOWER_H_RANGE.1)
        }
    }

    #[test]
    fn hour_more_changing_time() {
        let reactor = Reactor::new(52.091943, 47.951047, 1_000_000);
        let hour = 100500;
        let res = reactor.inject(hour);
        assert_eq!(res.len(), 1_000_000);

        for poly in res {
            assert!(poly.height > UPPER_H_RANGE.0 && poly.height < UPPER_H_RANGE.1)
        }
    }
}
