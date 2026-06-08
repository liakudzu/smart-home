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

// ==================== Модуль network (имитаторы и сетевые устройства) ====================
pub mod network {
    //! Сетевые умные устройства для взаимодействия с имитаторами
    use std::error::Error;
    use std::io::{BufRead, BufReader, Write};
    use std::net::{TcpStream, UdpSocket};
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    /// Умная розетка, управляемая по TCP
    pub struct NetworkSmartSocket {
        stream: TcpStream,
    }

    impl NetworkSmartSocket {
        pub fn connect(addr: &str) -> Result<Self, Box<dyn Error>> {
            let stream = TcpStream::connect(addr)?;
            stream.set_read_timeout(Some(Duration::from_secs(1)))?;
            stream.set_write_timeout(Some(Duration::from_secs(1)))?;
            Ok(NetworkSmartSocket { stream })
        }

        fn send_command(&mut self, cmd: &str) -> Result<String, Box<dyn Error>> {
            self.stream.write_all(cmd.as_bytes())?;
            self.stream.write_all(b"\n")?;
            let mut reader = BufReader::new(self.stream.try_clone()?);
            let mut resp = String::new();
            reader.read_line(&mut resp)?;
            Ok(resp.trim().to_string())
        }

        pub fn turn_on(&mut self) -> Result<(), Box<dyn Error>> {
            let resp = self.send_command("ON")?;
            if resp == "OK" {
                Ok(())
            } else {
                Err("некорректный ответ".into())
            }
        }

        pub fn turn_off(&mut self) -> Result<(), Box<dyn Error>> {
            let resp = self.send_command("OFF")?;
            if resp == "OK" {
                Ok(())
            } else {
                Err("некорректный ответ".into())
            }
        }

        pub fn is_on(&mut self) -> Result<bool, Box<dyn Error>> {
            let resp = self.send_command("STATE")?;
            Ok(resp == "ON")
        }

        pub fn current_power(&mut self) -> Result<f64, Box<dyn Error>> {
            let resp = self.send_command("POWER")?;
            Ok(resp.parse::<f64>()?)
        }
    }

    /// Умный термометр, получающий температуру по UDP в фоновом потоке
    pub struct NetworkSmartThermometer {
        receiver: mpsc::Receiver<f64>,
        _handle: thread::JoinHandle<()>,
        stop_flag: std::sync::Arc<std::sync::atomic::AtomicBool>,
    }

    impl NetworkSmartThermometer {
        /// Привязывается к локальному адресу для приёма UDP-пакетов
        pub fn bind(addr: &str) -> Result<Self, Box<dyn Error>> {
            let socket = UdpSocket::bind(addr)?;
            socket.set_nonblocking(true)?;
            let (tx, rx) = mpsc::channel();
            let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
            let stop_clone = stop.clone();
            let handle = thread::spawn(move || {
                let mut buf = [0u8; 1024];
                while !stop_clone.load(std::sync::atomic::Ordering::Relaxed) {
                    match socket.recv_from(&mut buf) {
                        Ok((size, _)) => {
                            let data = String::from_utf8_lossy(&buf[..size]);
                            if let Ok(temp) = data.trim().parse::<f64>() {
                                let _ = tx.send(temp);
                            }
                        }
                        Err(e) => {
                            if e.kind() == std::io::ErrorKind::WouldBlock {
                                std::thread::sleep(Duration::from_millis(10));
                                continue;
                            } else {
                                break;
                            }
                        }
                    }
                }
            });
            Ok(NetworkSmartThermometer {
                receiver: rx,
                _handle: handle,
                stop_flag: stop,
            })
        }

        /// Получить последнее значение температуры (неблокирующе, с таймаутом)
        pub fn get_temperature(&mut self) -> Result<f64, Box<dyn Error>> {
            // Пропускаем все накопленные значения, оставляем только последнее
            while self.receiver.try_recv().is_ok() {}
            match self.receiver.recv_timeout(Duration::from_millis(1000)) {
                Ok(temp) => Ok(temp),
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    Err("Данные о температуре не получены".into())
                }
                Err(_) => Err("Канал закрыт".into()),
            }
        }
    }

    impl Drop for NetworkSmartThermometer {
        fn drop(&mut self) {
            self.stop_flag
                .store(true, std::sync::atomic::Ordering::Relaxed);
            // Попытаться присоединить поток (по возможности, best-effort)
            let old = std::mem::replace(&mut self._handle, std::thread::spawn(|| {}));
            let _ = old.join();
        }
    }
}

// ==================== НОВЫЕ МОДУЛИ (паттерны) ====================
pub mod observer;
pub mod report_builder;
pub mod smart_house_builder;

// ==================== ТЕСТЫ ====================
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
