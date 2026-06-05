use smart_home::observer::ObservableRoom;
use smart_home::{SmartThermometer, SmartSocket, SmartDevice, Report};
use std::collections::HashMap;

fn main() {
    let mut room = ObservableRoom::new(HashMap::new());

    // Регистрация замыкания в качестве наблюдателя.
    room.register_callback(|name, device| {
        println!("[Callback] Добавлено новое устройство: '{}' - {}", name, device.report());
    });

    // Регистрация структуры, реализующей трейт Observer.
    struct Logger;
    impl smart_home::observer::Observer for Logger {
            fn update(&self, name: &str, device: &SmartDevice) {
            println!("[Логгер] Устройство '{}' добавлено: {}", name, device.report());
        }
    }
    room.register_observer(Box::new(Logger));

    println!("Observer Demo:");
    room.add_device("Socket".to_string(), SmartSocket::new(150.0).into());
    room.add_device("Thermometer".to_string(), SmartThermometer::new(23.0).into());
}
