use smart_home::{report_builder::ReportBuilder, SmartThermometer, SmartSocket, Room, SmartDevice, room};

fn main() {
    let thermo1 = SmartThermometer::new(20.0);
    let thermo2 = SmartThermometer::new(22.0);
    let socket1 = SmartSocket::new(100.0);
    let socket2 = SmartSocket::new(200.0);
    let room1 = room! {
        "device1" => SmartDevice::Thermometer(thermo1.clone()),
        "device2" => SmartDevice::Socket(socket1),
    };
    let room2 = room! {
        "device3" => SmartDevice::Thermometer(thermo2),
        "device4" => SmartDevice::Socket(socket2),
    };

    let reporter = ReportBuilder::new()
        .add_report(&thermo1)
        .add_report(&socket1)
        .add_report(&room1)
        .add_report(&room2);

    println!("Демонстрация сборщика отчётов:");
    reporter.report();
}