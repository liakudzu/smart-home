use smart_home::{SmartDevice, SmartHouse, SmartSocket, SmartThermometer, Room};

fn main() {
    // Создаём термометр
    let thermometer = SmartDevice::Thermometer(SmartThermometer::new(22.5));

    // Создаём розетку для гостиной и сразу включаем её
    let mut socket1 = SmartSocket::new(150.0); // мощность 150 Вт
    socket1.turn_on();                         // включаем
    let socket1 = SmartDevice::Socket(socket1);

    // Вторая розетка (в спальне) остаётся выключенной
    let socket2 = SmartDevice::Socket(SmartSocket::new(75.0));

    let living_room = Room::new(vec![thermometer, socket1]);
    let bedroom = Room::new(vec![socket2]);

    let mut house = SmartHouse::new(vec![living_room, bedroom]);

    println!("Первый отчёт (розетка в гостиной включена):");
    house.report();

    // Выключаем розетку в гостиной (индекс 1 в комнате 0)
    if let SmartDevice::Socket(socket) = house.get_room_mut(0).get_device_mut(1) {
        socket.turn_off();
        println!("\nРозетка в гостиной выключена.\n");
    }

    println!("Второй отчёт (после выключения):");
    house.report();
}