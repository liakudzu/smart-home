use crate::{Room, SmartDevice, SmartHouse};
use std::collections::HashMap;

// --- Состояния-маркеры для контроля на этапе компиляции ---
pub struct NoRoomsYet;
pub struct HasRooms;

// --- Билдер, параметризованный состоянием ---
/// Билдер для `SmartHouse`, реализующий паттерн type-state.
///
/// Билдер параметризуется маркером состояния (`NoRoomsYet` / `HasRooms`),
/// чтобы исключать неверные последовательности вызовов (например, добавление
/// устройств до создания хотя бы одной комнаты). Методы возвращают новый
/// экземпляр билдера; пометка `#[must_use]` помогает компилятору/линтерам
/// предупреждать об игнорировании возвращаемого значения.
pub struct SmartHouseBuilder<State = NoRoomsYet> {
    rooms: HashMap<String, Room>,
    current_room: Option<String>,
    _state: std::marker::PhantomData<State>,
}

impl SmartHouseBuilder<NoRoomsYet> {
    /// Создаёт новый билдер в начальном состоянии (без комнат).
    pub fn new() -> Self {
        SmartHouseBuilder {
            rooms: HashMap::new(),
            current_room: None,
            _state: std::marker::PhantomData,
        }
    }

    /// Добавляет новую комнату и возвращает билдер в состоянии `HasRooms`.
    /// Теперь можно добавлять устройства.
    ///
    /// Пометка `#[must_use]` помогает избежать ситуаций, когда вызов
    /// `.add_room(...)` проводится без использования возвращаемого результата.
    #[must_use = "Используйте возвращаемое значение билдера"]
    pub fn add_room(mut self, name: impl Into<String>) -> SmartHouseBuilder<HasRooms> {
        let name = name.into();
        let new_room = Room::new(HashMap::new());
        self.rooms.insert(name.clone(), new_room);
        SmartHouseBuilder {
            rooms: self.rooms,
            current_room: Some(name),
            _state: std::marker::PhantomData,
        }
    }
}

impl SmartHouseBuilder<HasRooms> {
    /// Добавляет устройство в последнюю добавленную комнату.
    ///
    /// Возвращает обновлённый билдер; не игнорируйте возвращаемое значение.
    #[must_use = "Используйте возвращаемое значение билдера"]
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

    /// Добавляет новую комнату и возвращает билдер в состоянии `HasRooms`.
    /// Позволяет вызывать `add_room` несколько раз подряд.
    ///
    /// Аналогично — возвращаемое значение следует использовать.
    #[must_use = "Используйте возвращаемое значение билдера"]
    pub fn add_room(mut self, name: impl Into<String>) -> Self {
        let name = name.into();
        let new_room = Room::new(std::collections::HashMap::new());
        self.rooms.insert(name.clone(), new_room);
        self.current_room = Some(name);
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
