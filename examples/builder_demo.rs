use smart_home::{smart_house_builder::SmartHouseBuilder, SmartThermometer, SmartSocket, Report};

fn main() {
    // Попытка добавить устройство без комнаты приведёт к ошибке компиляции:
    // let builder = SmartHouseBuilder::new().add_device("socket", SmartSocket::new(150.0).into());
    // error[E0599]: no method named `add_device` found for struct `SmartHouseBuilder<NoRoomsYet>`

    // Правильный порядок: сначала комната, потом устройства.
    let house = SmartHouseBuilder::new()
        .add_room("Living Room")
        .add_device("Main Socket", SmartSocket::new(150.0).into())
        .add_device("Thermometer", SmartThermometer::new(22.5).into())
        .add_room("Bedroom")
        .add_device("Lamp", SmartSocket::new(75.0).into())
        .build();

    println!("Демонстрация билдера умного дома:");
    println!("{}", house.report());
}
