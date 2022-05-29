use crate::room::Room;

#[derive(Debug)]
pub enum HomeError {
    AlreadyExistsRoom,
    NotFoundRoom,
}

type Result<T> = std::result::Result<T, HomeError>;

/// Дом
pub struct Home {
    name: String,
    rooms: Vec<Room>,
}

impl Home {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            rooms: vec![],
        }
    }

    /// Название дома
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Список помещений
    pub fn get_rooms(&self) -> &[Room] {
        &self.rooms
    }

    /// Добавлять помещение в доме
    pub fn add_room(&mut self, room: Room) -> Result<()> {
        if self.rooms.iter().any(|r| r.get_uid() == room.get_uid()) {
            return Err(HomeError::AlreadyExistsRoom);
        }
        self.rooms.push(room);
        Ok(())
    }

    /// Удалить помещение в доме
    pub fn remove_room(&mut self, uid: &str) -> Result<Room> {
        if let Some(index) = self.rooms.iter().position(|r| r.get_uid() == uid) {
            return Ok(self.rooms.remove(index));
        }

        Err(HomeError::NotFoundRoom)
    }

    /// Tекстовый отчёт о полном состоянии дома
    pub fn report(&self) -> String {
        let mut report = vec![];
        for room in self.rooms.iter() {
            report.push(format!("{}:", room.get_uid()));
            for device in room.get_devices() {
                report.push(format!("\t{}", device.description()));
            }
        }

        report.join("\n")
    }
}
