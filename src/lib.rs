//! Библиотека "Умный дом" (расширенная версия)

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

pub trait Report {
    fn report(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct SmartSocket {
    power: f64,
    is_on: bool,
}

impl SmartSocket {
    pub fn new(power: f64) -> Self {
        Self {
            power,
            is_on: false,
        }
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
    }

    pub fn turn_off(&mut self) {
        self.is_on = false;
    }

    pub fn is_on(&self) -> bool {
        self.is_on
    }

    pub fn current_power(&self) -> f64 {
        if self.is_on {
            self.power
        } else {
            0.0
        }
    }
}

impl Report for SmartSocket {
    fn report(&self) -> String {
        format!(
            "Smart socket: state={}, power={} W",
            if self.is_on { "on" } else { "off" },
            self.current_power()
        )
    }
}

#[derive(Debug, Clone)]
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

    pub fn set_temperature(&mut self, temperature: f64) {
        self.temperature = temperature;
    }
}

impl Report for SmartThermometer {
    fn report(&self) -> String {
        format!("Smart thermometer: {:.1} C", self.temperature)
    }
}

#[derive(Debug, Clone)]
pub enum SmartDevice {
    Socket(SmartSocket),
    Thermometer(SmartThermometer),
}

impl Report for SmartDevice {
    fn report(&self) -> String {
        match self {
            SmartDevice::Socket(device) => device.report(),
            SmartDevice::Thermometer(device) => device.report(),
        }
    }
}

impl From<SmartSocket> for SmartDevice {
    fn from(value: SmartSocket) -> Self {
        SmartDevice::Socket(value)
    }
}

impl From<SmartThermometer> for SmartDevice {
    fn from(value: SmartThermometer) -> Self {
        SmartDevice::Thermometer(value)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Room {
    devices: HashMap<String, SmartDevice>,
}

impl Room {
    pub fn new(devices: HashMap<String, SmartDevice>) -> Self {
        Self { devices }
    }

    pub fn add_device(&mut self, name: String, device: SmartDevice) -> Option<SmartDevice> {
        self.devices.insert(name, device)
    }

    pub fn remove_device(&mut self, name: &str) -> Option<SmartDevice> {
        self.devices.remove(name)
    }

    pub fn get_device(&self, name: &str) -> Option<&SmartDevice> {
        self.devices.get(name)
    }
}

impl Report for Room {
    fn report(&self) -> String {
        let mut lines = vec![format!("Room devices: {}", self.devices.len())];
        for (name, device) in &self.devices {
            lines.push(format!("- {}: {}", name, device.report()));
        }
        lines.join("\n")
    }
}

#[derive(Debug, Clone)]
pub struct SmartHouse {
    rooms: HashMap<String, Room>,
}

impl SmartHouse {
    pub fn new(rooms: HashMap<String, Room>) -> Self {
        Self { rooms }
    }

    pub fn add_room(&mut self, name: String, room: Room) -> Option<Room> {
        self.rooms.insert(name, room)
    }

    pub fn get_room_mut(&mut self, name: &str) -> Option<&mut Room> {
        self.rooms.get_mut(name)
    }

    pub fn get_device(
        &self,
        room_name: &str,
        device_name: &str,
    ) -> Result<&SmartDevice, SmartHouseError> {
        let room = self
            .rooms
            .get(room_name)
            .ok_or_else(|| SmartHouseError::RoomNotFound(room_name.to_string()))?;

        room.get_device(device_name)
            .ok_or_else(|| SmartHouseError::DeviceNotFound {
                room: room_name.to_string(),
                device: device_name.to_string(),
            })
    }
}

impl Report for SmartHouse {
    fn report(&self) -> String {
        let mut lines = vec![format!("Smart house rooms: {}", self.rooms.len())];
        for (name, room) in &self.rooms {
            lines.push(format!("{}:\n{}", name, room.report()));
        }
        lines.join("\n")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmartHouseError {
    RoomNotFound(String),
    DeviceNotFound { room: String, device: String },
}

impl fmt::Display for SmartHouseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SmartHouseError::RoomNotFound(room) => write!(f, "room '{}' not found", room),
            SmartHouseError::DeviceNotFound { room, device } => {
                write!(f, "device '{}' not found in room '{}'", device, room)
            }
        }
    }
}

impl Error for SmartHouseError {}

#[macro_export]
macro_rules! room {
    ($($name:expr => $device:expr),* $(,)?) => {{
        let mut devices = ::std::collections::HashMap::new();
        $(
            devices.insert($name.to_string(), $crate::SmartDevice::from($device));
        )*
        $crate::Room::new(devices)
    }};
}

// Конец старого кода. Далее добавляем новый модуль network.

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
        reader: BufReader<TcpStream>,
    }

    impl NetworkSmartSocket {
        pub fn connect(addr: &str) -> Result<Self, Box<dyn Error>> {
            let stream = TcpStream::connect(addr)?;
            stream.set_read_timeout(Some(Duration::from_secs(1)))?;
            stream.set_write_timeout(Some(Duration::from_secs(1)))?;
            let reader_stream = stream.try_clone()?;
            Ok(NetworkSmartSocket {
                stream,
                reader: BufReader::new(reader_stream),
            })
        }

        fn send_command(&mut self, cmd: &str) -> Result<String, Box<dyn Error>> {
            self.stream.write_all(cmd.as_bytes())?;
            self.stream.write_all(b"\n")?;
            self.stream.flush()?;

            let mut response = String::new();
            self.reader.read_line(&mut response)?;
            Ok(response.trim().to_string())
        }

        pub fn turn_on(&mut self) -> Result<(), Box<dyn Error>> {
            let resp = self.send_command("ON")?;
            if resp == "OK" {
                Ok(())
            } else {
                Err("invalid response".into())
            }
        }

        pub fn turn_off(&mut self) -> Result<(), Box<dyn Error>> {
            let resp = self.send_command("OFF")?;
            if resp == "OK" {
                Ok(())
            } else {
                Err("invalid response".into())
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
        last_temperature: Option<f64>,
        _handle: thread::JoinHandle<()>,
    }

    impl NetworkSmartThermometer {
        /// Привязывается к локальному адресу для приёма UDP-пакетов
        pub fn bind(addr: &str) -> Result<Self, Box<dyn Error>> {
            let socket = UdpSocket::bind(addr)?;
            let (tx, rx) = mpsc::channel();
            let handle = thread::spawn(move || {
                let mut buf = [0u8; 1024];
                while let Ok((size, _)) = socket.recv_from(&mut buf) {
                    let data = String::from_utf8_lossy(&buf[..size]);
                    if let Ok(temp) = data.trim().parse::<f64>() {
                        let _ = tx.send(temp);
                    }
                }
            });
            Ok(NetworkSmartThermometer {
                receiver: rx,
                last_temperature: None,
                _handle: handle,
            })
        }

        /// Получить последнее значение температуры (неблокирующе, с таймаутом)
        pub fn get_temperature(&mut self) -> Result<f64, Box<dyn Error>> {
            while let Ok(temp) = self.receiver.try_recv() {
                self.last_temperature = Some(temp);
            }

            if let Some(temp) = self.last_temperature {
                return Ok(temp);
            }

            match self.receiver.recv_timeout(Duration::from_millis(1_100)) {
                Ok(temp) => {
                    self.last_temperature = Some(temp);
                    Ok(temp)
                }
                Err(mpsc::RecvTimeoutError::Timeout) => Err("No temperature data received".into()),
                Err(_) => Err("Channel closed".into()),
            }
        }
    }
}
