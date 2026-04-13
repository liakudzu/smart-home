//! Библиотека "Умный дом" (расширенная версия)

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

// ==================== Трейт Report ====================
pub trait Report {
    fn report(&self) -> String;
}

// ==================== Умный термометр ====================
#[derive(Debug, Clone, PartialEq)]
pub struct SmartThermometer {
    temperature: f64,
}

impl SmartThermometer {
    pub fn new(temperature: f64) -> Self {
        Self { temperature }
    }

    pub fn temperature(&self) -> f64 {
        self.temperature
    }
}

impl Report for SmartThermometer {
    fn report(&self) -> String {
        format!("Термометр: температура = {}°C", self.temperature)
    }
}

// ==================== Умная розетка ====================
#[derive(Debug, Clone, PartialEq)]
pub struct SmartSocket {
    enabled: bool,
    power_when_on: f64,
}

impl SmartSocket {
    pub fn new(power_when_on: f64) -> Self {
        Self {
            enabled: false,
            power_when_on,
        }
    }

    pub fn turn_on(&mut self) {
        self.enabled = true;
    }

    pub fn turn_off(&mut self) {
        self.enabled = false;
    }

    pub fn is_on(&self) -> bool {
        self.enabled
    }

    pub fn current_power(&self) -> f64 {
        if self.enabled {
            self.power_when_on
        } else {
            0.0
        }
    }
}

impl Report for SmartSocket {
    fn report(&self) -> String {
        let state = if self.enabled {
            "включена"
        } else {
            "выключена"
        };
        format!("Розетка: {}, мощность = {} Вт", state, self.current_power())
    }
}

// ==================== Умное устройство (enum) ====================
#[derive(Debug, Clone, PartialEq)]
pub enum SmartDevice {
    Thermometer(SmartThermometer),
    Socket(SmartSocket),
}

impl Report for SmartDevice {
    fn report(&self) -> String {
        match self {
            SmartDevice::Thermometer(t) => t.report(),
            SmartDevice::Socket(s) => s.report(),
        }
    }
}

// Реализация From для преобразования в SmartDevice
impl From<SmartThermometer> for SmartDevice {
    fn from(t: SmartThermometer) -> Self {
        SmartDevice::Thermometer(t)
    }
}

impl From<SmartSocket> for SmartDevice {
    fn from(s: SmartSocket) -> Self {
        SmartDevice::Socket(s)
    }
}

// ==================== Тип ошибки ====================
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceLookupError {
    RoomNotFound(String),
    DeviceNotFound(String),
}

impl fmt::Display for DeviceLookupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceLookupError::RoomNotFound(name) => write!(f, "Комната '{}' не найдена", name),
            DeviceLookupError::DeviceNotFound(name) => {
                write!(f, "Устройство '{}' не найдено", name)
            }
        }
    }
}

impl Error for DeviceLookupError {}

// ==================== Комната ====================
#[derive(Debug, Clone, PartialEq)]
pub struct Room {
    devices: HashMap<String, SmartDevice>,
}

impl Room {
    pub fn new(devices: HashMap<String, SmartDevice>) -> Self {
        Self { devices }
    }

    // Получить ссылку на устройство по ключу (возвращает Option)
    pub fn get_device(&self, name: &str) -> Option<&SmartDevice> {
        self.devices.get(name)
    }

    // Получить мутабельную ссылку на устройство по ключу
    pub fn get_device_mut(&mut self, name: &str) -> Option<&mut SmartDevice> {
        self.devices.get_mut(name)
    }

    // Добавить устройство
    pub fn add_device(&mut self, name: String, device: SmartDevice) {
        self.devices.insert(name, device);
    }

    // Удалить устройство
    pub fn remove_device(&mut self, name: &str) -> Option<SmartDevice> {
        self.devices.remove(name)
    }

    // Получить итератор по устройствам (для отчёта)
    pub fn devices(&self) -> impl Iterator<Item = (&String, &SmartDevice)> {
        self.devices.iter()
    }
}

impl Report for Room {
    fn report(&self) -> String {
        let mut s = format!("Комната (устройств: {}):", self.devices.len());
        for (name, device) in &self.devices {
            s.push_str(&format!("\n  - {}: {}", name, device.report()));
        }
        s
    }
}

// ==================== Умный дом ====================
#[derive(Debug, Clone, PartialEq)]
pub struct SmartHouse {
    rooms: HashMap<String, Room>,
}

impl SmartHouse {
    pub fn new(rooms: HashMap<String, Room>) -> Self {
        Self { rooms }
    }

    // Получить ссылку на комнату по ключу
    pub fn get_room(&self, name: &str) -> Option<&Room> {
        self.rooms.get(name)
    }

    // Получить мутабельную ссылку на комнату по ключу
    pub fn get_room_mut(&mut self, name: &str) -> Option<&mut Room> {
        self.rooms.get_mut(name)
    }

    // Добавить комнату
    pub fn add_room(&mut self, name: String, room: Room) {
        self.rooms.insert(name, room);
    }

    // Удалить комнату
    pub fn remove_room(&mut self, name: &str) -> Option<Room> {
        self.rooms.remove(name)
    }

    // Получить ссылку на устройство по имени комнаты и имени устройства
    // Возвращает Result, а не Option
    pub fn get_device(
        &self,
        room_name: &str,
        device_name: &str,
    ) -> Result<&SmartDevice, DeviceLookupError> {
        let room = self
            .rooms
            .get(room_name)
            .ok_or_else(|| DeviceLookupError::RoomNotFound(room_name.to_string()))?;
        room.get_device(device_name)
            .ok_or_else(|| DeviceLookupError::DeviceNotFound(device_name.to_string()))
    }

    // Мутабельная версия
    pub fn get_device_mut(
        &mut self,
        room_name: &str,
        device_name: &str,
    ) -> Result<&mut SmartDevice, DeviceLookupError> {
        let room = self
            .rooms
            .get_mut(room_name)
            .ok_or_else(|| DeviceLookupError::RoomNotFound(room_name.to_string()))?;
        room.get_device_mut(device_name)
            .ok_or_else(|| DeviceLookupError::DeviceNotFound(device_name.to_string()))
    }
}

impl Report for SmartHouse {
    fn report(&self) -> String {
        let mut s = String::from("=== УМНЫЙ ДОМ ===\n");
        for (name, room) in &self.rooms {
            s.push_str(&format!("\nКомната '{}':\n", name));
            s.push_str(&room.report());
            s.push('\n');
        }
        s
    }
}

// ==================== Макрос для создания комнаты ====================
#[macro_export]
macro_rules! room {
    ($($key:expr => $device:expr),* $(,)?) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key.into(), $device.into());
        )*
        $crate::Room::new(map)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermometer() {
        let t = SmartThermometer::new(20.0);
        assert_eq!(t.temperature(), 20.0);
        assert_eq!(t.report(), "Термометр: температура = 20°C");
    }

    #[test]
    fn test_socket() {
        let mut s = SmartSocket::new(100.0);
        assert!(!s.is_on());
        assert_eq!(s.current_power(), 0.0);
        s.turn_on();
        assert!(s.is_on());
        assert_eq!(s.current_power(), 100.0);
        assert_eq!(s.report(), "Розетка: включена, мощность = 100 Вт");
        s.turn_off();
        assert_eq!(s.report(), "Розетка: выключена, мощность = 0 Вт");
    }

    #[test]
    fn test_room_operations() {
        let mut room = Room::new(HashMap::new());
        room.add_device("dev1".to_string(), SmartThermometer::new(25.0).into());
        assert!(room.get_device("dev1").is_some());
        assert!(room.get_device("dev2").is_none());
        let removed = room.remove_device("dev1");
        assert!(removed.is_some());
        assert!(room.get_device("dev1").is_none());
    }

    #[test]
    fn test_house_operations() {
        let mut house = SmartHouse::new(HashMap::new());
        let room = Room::new(HashMap::new());
        house.add_room("room1".to_string(), room);
        assert!(house.get_room("room1").is_some());
        assert!(house.get_room("room2").is_none());

        house.remove_room("room1");
        assert!(house.get_room("room1").is_none());
    }

    #[test]
    fn test_get_device_result() {
        let mut room = Room::new(HashMap::new());
        room.add_device("socket".to_string(), SmartSocket::new(50.0).into());
        let mut rooms = HashMap::new();
        rooms.insert("living".to_string(), room);
        let house = SmartHouse::new(rooms);

        assert!(house.get_device("living", "socket").is_ok());
        assert!(matches!(
            house.get_device("living", "unknown"),
            Err(DeviceLookupError::DeviceNotFound(_))
        ));
        assert!(matches!(
            house.get_device("unknown", "socket"),
            Err(DeviceLookupError::RoomNotFound(_))
        ));
    }

    #[test]
    fn test_macro() {
        let room = room! {
            "t" => SmartThermometer::new(30.0),
            "s" => SmartSocket::new(200.0),
        };
        assert!(room.get_device("t").is_some());
        assert!(room.get_device("s").is_some());
        assert_eq!(room.devices().count(), 2);
    }

    #[test]
    fn test_from_traits() {
        let t = SmartThermometer::new(15.0);
        let dev: SmartDevice = t.into();
        match dev {
            SmartDevice::Thermometer(therm) => assert_eq!(therm.temperature(), 15.0),
            _ => panic!("wrong type"),
        }

        let s = SmartSocket::new(300.0);
        let dev: SmartDevice = s.into();
        match dev {
            SmartDevice::Socket(sock) => assert!(!sock.is_on()),
            _ => panic!("wrong type"),
        }
    }

    #[test]
    fn test_debug() {
        let t = SmartThermometer::new(10.0);
        let debug_str = format!("{:?}", t);
        assert!(debug_str.contains("10"));
        let s = SmartSocket::new(50.0);
        let debug_str = format!("{:?}", s);
        assert!(debug_str.contains("50"));
        let room = room! {"dev" => t};
        let debug_str = format!("{:?}", room);
        assert!(debug_str.contains("dev"));
    }
}
