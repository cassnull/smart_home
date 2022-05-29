pub mod smart_socket;
pub mod thermometer;

use self::smart_socket::SmartSocket;
use self::thermometer::Thermometer;

/// Устройство
pub trait Device {
    fn get_id(&self) -> &str;

    fn description(&self) -> String;
}

impl Device for SmartSocket {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn description(&self) -> String {
        format!(
            "{}: потребляемая мощность {}",
            self,
            self.get_current_power_consumption()
        )
    }
}

impl Device for Thermometer {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn description(&self) -> String {
        format!("{}: температура {}", self, self.get_current_temperature())
    }
}
