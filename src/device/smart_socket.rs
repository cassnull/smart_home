use std::fmt;

/// Умная розетка
pub struct SmartSocket {
    pub id: String,
    power_consumption: f64,
    power: f64,
}

impl fmt::Display for SmartSocket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Умная розетка {}", self.id)
    }
}

impl SmartSocket {
    pub fn new(id: &str, power_consumption: f64) -> Self {
        Self {
            id: id.to_owned(),
            power_consumption,
            power: 0.,
        }
    }

    /// Включить
    pub fn on(&mut self) {
        self.power = 220.;
    }

    /// Выключить
    pub fn off(&mut self) {
        self.power = 0.;
    }

    /// Текущая потребляемая мощность
    pub fn get_current_power_consumption(&self) -> f64 {
        self.power_consumption
    }
}
