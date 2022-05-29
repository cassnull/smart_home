use crate::device::Device;

#[derive(Debug)]
pub enum RoomError {
    AlreadyExistsDevice,
    NotFoundDevice,
}

type Result<T> = std::result::Result<T, RoomError>;

/// Помещение
pub struct Room {
    uid: String,
    devices: Vec<Box<dyn Device>>,
}

impl Room {
    pub fn new(name: &str) -> Self {
        Self {
            uid: name.into(),
            devices: vec![],
        }
    }
    /// Уникальное название
    pub fn get_uid(&self) -> &str {
        &self.uid
    }

    /// Список устройств
    pub fn get_devices(&self) -> &[Box<dyn Device>] {
        &self.devices
    }

    /// Добавлять устройство в помещении
    pub fn add_device(&mut self, device: impl Device + 'static) -> Result<()> {
        if self.devices.iter().any(|d| d.get_id() == device.get_id()) {
            return Err(RoomError::AlreadyExistsDevice);
        }
        self.devices.push(Box::new(device));
        Ok(())
    }

    /// Удалить устройство в помещении
    pub fn remove_device(&mut self, id: &str) -> Result<Box<dyn Device>> {
        if let Some(index) = self.devices.iter().position(|d| d.get_id() == id) {
            return Ok(self.devices.remove(index));
        }

        Err(RoomError::NotFoundDevice)
    }
}
