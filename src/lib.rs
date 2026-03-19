//! Библиотека "Умный дом"

/// Умный термометр.
pub struct SmartThermometer {
    temperature: f64,
}

impl SmartThermometer {
    /// Создаёт новый термометр с заданной температурой.
    pub fn new(temperature: f64) -> Self {
        Self { temperature }
    }

    /// Возвращает текущую температуру.
    pub fn temperature(&self) -> f64 {
        self.temperature
    }
}

/// Умная розетка.
pub struct SmartSocket {
    enabled: bool,
    /// Потребляемая мощность во включённом состоянии (произвольное число).
    power_when_on: f64,
}

impl SmartSocket {
    /// Создаёт новую розетку в выключенном состоянии.
    /// Можно задать мощность, которая будет возвращаться при включении.
    pub fn new(power_when_on: f64) -> Self {
        Self {
            enabled: false,
            power_when_on,
        }
    }

    /// Включить розетку.
    pub fn turn_on(&mut self) {
        self.enabled = true;
    }

    /// Выключить розетку.
    pub fn turn_off(&mut self) {
        self.enabled = false;
    }

    /// Проверить, включена ли розетка.
    pub fn is_on(&self) -> bool {
        self.enabled
    }

    /// Возвращает текущую потребляемую мощность.
    /// Если розетка выключена, возвращает 0, иначе заданную мощность.
    pub fn current_power(&self) -> f64 {
        if self.enabled {
            self.power_when_on
        } else {
            0.0
        }
    }
}

/// Умное устройство – может быть либо термометром, либо розеткой.
pub enum SmartDevice {
    Thermometer(SmartThermometer),
    Socket(SmartSocket),
}

impl SmartDevice {
    /// Выводит в стандартный вывод описание состояния устройства.
    pub fn describe(&self) {
        match self {
            SmartDevice::Thermometer(t) => {
                println!("Термометр: температура = {}°C", t.temperature());
            }
            SmartDevice::Socket(s) => {
                let state = if s.is_on() { "включена" } else { "выключена" };
                println!("Розетка: {}, мощность = {} Вт", state, s.current_power());
            }
        }
    }
}

/// Комната, содержащая список умных устройств.
pub struct Room {
    devices: Vec<SmartDevice>,
}

impl Room {
    /// Создаёт новую комнату с заданным списком устройств.
    pub fn new(devices: Vec<SmartDevice>) -> Self {
        Self { devices }
    }

    /// Возвращает ссылку на устройство по индексу.
    /// Паникует, если индекс вне допустимого диапазона.
    pub fn get_device(&self, index: usize) -> &SmartDevice {
        &self.devices[index]
    }

    /// Возвращает мутабельную ссылку на устройство по индексу.
    /// Паникует, если индекс вне допустимого диапазона.
    pub fn get_device_mut(&mut self, index: usize) -> &mut SmartDevice {
        &mut self.devices[index]
    }

    /// Выводит в стандартный вывод отчёт обо всех устройствах в комнате.
    pub fn report(&self) {
        println!("Отчёт по комнате (устройств: {}):", self.devices.len());
        for (i, device) in self.devices.iter().enumerate() {
            print!("  [{}] ", i);
            device.describe();
        }
    }
}

/// Умный дом, содержащий список комнат.
pub struct SmartHouse {
    rooms: Vec<Room>,
}

impl SmartHouse {
    /// Создаёт новый дом с заданным списком комнат.
    pub fn new(rooms: Vec<Room>) -> Self {
        Self { rooms }
    }

    /// Возвращает ссылку на комнату по индексу.
    /// Паникует, если индекс вне допустимого диапазона.
    pub fn get_room(&self, index: usize) -> &Room {
        &self.rooms[index]
    }

    /// Возвращает мутабельную ссылку на комнату по индексу.
    /// Паникует, если индекс вне допустимого диапазона.
    pub fn get_room_mut(&mut self, index: usize) -> &mut Room {
        &mut self.rooms[index]
    }

    /// Выводит в стандартный вывод отчёт обо всех комнатах в доме.
    pub fn report(&self) {
        println!("=== УМНЫЙ ДОМ ===\n");
        for (i, room) in self.rooms.iter().enumerate() {
            println!("Комната #{}:", i);
            room.report();
            println!();
        }
    }
}
