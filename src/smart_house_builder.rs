use crate::{Room, SmartDevice, SmartHouse};
use std::collections::HashMap;

// --- Состояния-маркеры для контроля на этапе компиляции ---
pub struct NoRoomsYet;
pub struct HasRooms;

// --- Билдер, параметризованный состоянием ---
pub struct SmartHouseBuilder<State = NoRoomsYet> {
    rooms: HashMap<String, Room>,
    devices_buffer: HashMap<String, SmartDevice>,
    current_room: Option<String>,
    _state: std::marker::PhantomData<State>,
}

impl SmartHouseBuilder<NoRoomsYet> {
    /// Создаёт новый билдер в начальном состоянии (без комнат).
    pub fn new() -> Self {
        SmartHouseBuilder {
            rooms: HashMap::new(),
            devices_buffer: HashMap::new(),
            current_room: None,
            _state: std::marker::PhantomData,
        }
    }

    /// Добавляет новую комнату и возвращает билдер в состоянии `HasRooms`.
    /// Теперь можно добавлять устройства.
    pub fn add_room(mut self, name: impl Into<String>) -> SmartHouseBuilder<HasRooms> {
        let name = name.into();
        let new_room = Room::new(HashMap::new());
        self.rooms.insert(name.clone(), new_room);
        SmartHouseBuilder {
            rooms: self.rooms,
            devices_buffer: self.devices_buffer,
            current_room: Some(name),
            _state: std::marker::PhantomData,
        }
    }
}

impl SmartHouseBuilder<HasRooms> {
    /// Добавляет устройство в последнюю добавленную комнату.
    pub fn add_device(mut self, name: impl Into<String>, device: SmartDevice) -> Self {
        let device_name = name.into();
        if let Some(room_name) = &self.current_room {
            if let Some(room) = self.rooms.get_mut(room_name) {
                room.add_device(device_name, device);
            } else {
                eprintln!("Ошибка: не удалось найти комнату '{}'", room_name);
            }
        }
        self
    }

    /// Завершает сборку и возвращает готовый объект `SmartHouse`.
    pub fn build(self) -> SmartHouse {
        SmartHouse::new(self.rooms)
    }
}

impl Default for SmartHouseBuilder<NoRoomsYet> {
    fn default() -> Self {
        Self::new()
    }
}
