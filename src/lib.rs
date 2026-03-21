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
#[cfg(test)]
mod tests {
    use super::*;

    // 1. Тесты для SmartThermometer
    #[test]
    fn thermometer_constructor_and_temperature() {
        let t = SmartThermometer::new(23.5);
        assert_eq!(t.temperature(), 23.5);
    }

    // 2. Тесты для SmartSocket
    #[test]
    fn socket_initial_state() {
        let s = SmartSocket::new(100.0);
        assert!(!s.is_on());
        assert_eq!(s.current_power(), 0.0);
    }

    #[test]
    fn socket_turn_on() {
        let mut s = SmartSocket::new(100.0);
        s.turn_on();
        assert!(s.is_on());
        assert_eq!(s.current_power(), 100.0);
    }

    #[test]
    fn socket_turn_off() {
        let mut s = SmartSocket::new(100.0);
        s.turn_on();
        s.turn_off();
        assert!(!s.is_on());
        assert_eq!(s.current_power(), 0.0);
    }

    // 3. Тесты для SmartDevice (проверка вариантов перечисления)
    #[test]
    fn device_enum_variants() {
        let t = SmartDevice::Thermometer(SmartThermometer::new(22.0));
        let s = SmartDevice::Socket(SmartSocket::new(150.0));

        match t {
            SmartDevice::Thermometer(therm) => assert_eq!(therm.temperature(), 22.0),
            _ => panic!("Wrong variant"),
        }

        match s {
            SmartDevice::Socket(socket) => assert!(!socket.is_on()),
            _ => panic!("Wrong variant"),
        }
    }

    // 4. Тесты для Room
    #[test]
    fn room_get_device() {
        let t = SmartDevice::Thermometer(SmartThermometer::new(21.0));
        let s = SmartDevice::Socket(SmartSocket::new(200.0));
        let room = Room::new(vec![t, s]);

        match room.get_device(0) {
            SmartDevice::Thermometer(therm) => assert_eq!(therm.temperature(), 21.0),
            _ => panic!("Expected thermometer"),
        }
        match room.get_device(1) {
            SmartDevice::Socket(socket) => assert!(!socket.is_on()),
            _ => panic!("Expected socket"),
        }
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn room_get_device_out_of_bounds() {
        let room = Room::new(vec![]);
        room.get_device(0); // должен паниковать
    }

    #[test]
    fn room_get_device_mut() {
        let mut room = Room::new(vec![SmartDevice::Socket(SmartSocket::new(50.0))]);
        {
            let dev = room.get_device_mut(0);
            if let SmartDevice::Socket(s) = dev {
                s.turn_on();
            }
        }
        // проверяем, что включилось
        if let SmartDevice::Socket(s) = room.get_device(0) {
            assert!(s.is_on());
            assert_eq!(s.current_power(), 50.0);
        } else {
            panic!("Not a socket");
        }
    }

    // 5. Тесты для SmartHouse
    #[test]
    fn house_get_room() {
        let room1 = Room::new(vec![]);
        let room2 = Room::new(vec![]);
        let house = SmartHouse::new(vec![room1, room2]);

        // просто проверяем, что метод не паникует
        let _r1 = house.get_room(0);
        let _r2 = house.get_room(1);
    }

    #[test]
    #[should_panic]
    fn house_get_room_out_of_bounds() {
        let house = SmartHouse::new(vec![]);
        house.get_room(0);
    }

    #[test]
    fn house_get_room_mut() {
        let mut house = SmartHouse::new(vec![Room::new(vec![])]);
        let _room_mut = house.get_room_mut(0);
        // Достаточно того, что метод вернул мутабельную ссылку и не запаниковал.
        // Если нужно проверить изменение, можно добавить устройство (если есть метод push)
    }

    // 6. Тест для describe (просто убеждаемся, что метод не паникует)
    #[test]
    fn device_describe_no_panic() {
        let t = SmartDevice::Thermometer(SmartThermometer::new(18.0));
        let s = SmartDevice::Socket(SmartSocket::new(120.0));
        t.describe();
        s.describe();
        // Если дошли до этой строки, тест пройден
    }

    // 7. Проверка включения/выключения розетки через мутабельную ссылку в доме
    #[test]
    fn turn_off_socket_in_room() {
        let mut socket = SmartSocket::new(75.0);
        socket.turn_on();
        let device = SmartDevice::Socket(socket);
        let room = Room::new(vec![device]);
        let mut house = SmartHouse::new(vec![room]);

        // получаем мутабельную ссылку на розетку и выключаем
        if let SmartDevice::Socket(s) = house.get_room_mut(0).get_device_mut(0) {
            s.turn_off();
        }

        // проверяем
        if let SmartDevice::Socket(s) = house.get_room(0).get_device(0) {
            assert!(!s.is_on());
            assert_eq!(s.current_power(), 0.0);
        } else {
            panic!("Device is not a socket");
        }
    }
}
