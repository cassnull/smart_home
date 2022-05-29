use std::fmt;

/// Термометр
pub struct Thermometer {
    pub id: String,
}

impl fmt::Display for Thermometer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Термометр {}", self.id)
    }
}

impl Thermometer {
    pub fn new(id: &str) -> Self {
        Self { id: id.to_owned() }
    }

    /// Текущая температура
    pub fn get_current_temperature(&self) -> f64 {
        25.
    }
}
