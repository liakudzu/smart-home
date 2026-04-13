use smart_home::{Report, SmartHouse, SmartSocket, SmartThermometer, room};
use std::collections::HashMap;

fn main() {
    // 1. Создаём устройства
    let thermometer = SmartThermometer::new(22.5);
    let socket1 = SmartSocket::new(150.0);
    let socket2 = SmartSocket::new(75.0);

    // 2. Создаём комнаты с помощью макроса
    let living_room = room! {
        "thermometer" => thermometer,
        "main_socket" => socket1,
    };
    let bedroom = room! {
        "socket" => socket2,
    };

    // 3. Создаём дом
    let mut rooms = HashMap::new();
    rooms.insert("living".to_string(), living_room);
    rooms.insert("bedroom".to_string(), bedroom);
    let mut house = SmartHouse::new(rooms);

    // 4. Выводим отчёт через универсальную функцию
    print_report(&house);
    println!("\n--- Добавляем новую комнату ---");
    let office = room! {
        "lamp_socket" => SmartSocket::new(40.0),
        "temp" => SmartThermometer::new(21.0),
    };
    house.add_room("office".to_string(), office);
    print_report(&house);

    println!("\n--- Добавляем устройство в существующую комнату ---");
    if let Some(living) = house.get_room_mut("living") {
        living.add_device("extra_socket".to_string(), SmartSocket::new(100.0).into());
    }
    print_report(&house);

    println!("\n--- Удаляем устройство из комнаты ---");
    if let Some(bedroom) = house.get_room_mut("bedroom") {
        bedroom.remove_device("socket");
    }
    print_report(&house);

    println!("\n--- Пытаемся получить несуществующее устройство ---");
    match house.get_device("living", "nonexistent") {
        Ok(dev) => println!("Устройство: {}", dev.report()),
        Err(e) => println!("Ошибка: {}", e),
    }

    match house.get_device("nonexistent_room", "socket") {
        Ok(dev) => println!("Устройство: {}", dev.report()),
        Err(e) => println!("Ошибка: {}", e),
    }

    println!("\n--- Демонстрация работы с отчётом для отдельных объектов ---");
    let some_socket = SmartSocket::new(200.0);
    print_report(&some_socket);
    let some_thermometer = SmartThermometer::new(18.0);
    print_report(&some_thermometer);
}

// Функция, принимающая любой объект, реализующий Report
fn print_report<R: Report>(item: &R) {
    println!("{}", item.report());
}
