#![allow(dead_code)]

pub struct Polygon {
    pub latitude: f32,  // широта
    pub longitude: f32, // долгота
    pub height: f32,    // высота поднятия
    pub d: f32,
    pub composition: Option<&'static str>,
    pub radiation: Option<&'static str>,
}

/// Полигон частиц. Используется из-за невозможности моделирования каждой частицы.
impl Polygon {
    pub fn new(latitude: f32, longitude: f32, height: f32, d: f32) -> Self {
        Polygon {
            latitude,
            longitude,
            height,
            d,
            composition: None,
            radiation: None,
        }
    }
}

/// Облако частиц - формация всех полигонов, испущеных реактором.
pub struct Cloud(Vec<Polygon>);

impl Cloud {
    pub fn new() -> Self {
        Cloud(Vec::new())
    }

    pub fn get_size(&self) -> usize {
        self.0.len()
    }

    pub fn add(&mut self, poly: Polygon) {
        self.0.push(poly);
    }

    pub fn extend(&mut self, new_injected: Vec<Polygon>) {
        self.0.extend(new_injected);
    }
}
