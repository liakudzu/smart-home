use smart_home::network::{NetworkSmartSocket, NetworkSmartThermometer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Адреса имитаторов (должны совпадать с запущенными эмуляторами)
    let socket_addr = "127.0.0.1:1234";
    let thermometer_addr = "127.0.0.1:8888";

    // Подключаемся к устройствам
    let mut socket = NetworkSmartSocket::connect(socket_addr)?;
    let mut thermometer = NetworkSmartThermometer::bind(thermometer_addr)?;

    // Включаем розетку
    socket.turn_on()?;
    println!("Socket turned on. Power: {} W", socket.current_power()?);
    // Получаем температуру
    match thermometer.get_temperature() {
        Ok(t) => println!("Current temperature: {:.2}°C", t),
        Err(e) => println!("Error reading temperature: {}", e),
    }

    // Создаём умный дом со старыми типами (используем моки) – для демонстрации отчёта.
    // Но мы хотим показать именно реальные устройства. Поэтому создадим свою структуру
    // для отображения состояния. Воспользуемся функцией print_report, которая требует Report.
    // NetworkSmartSocket и NetworkSmartThermometer не реализуют Report. Сделаем для них адаптер.

    // Для простоты выведем состояние вручную.
    println!("\n=== Состояние дома ===");
    println!(
        "Розетка: {}",
        if socket.is_on()? {
            "включена"
        } else {
            "выключена"
        }
    );
    println!("Мощность: {} Вт", socket.current_power()?);
    match thermometer.get_temperature() {
        Ok(t) => println!("Термометр: {:.2}°C", t),
        Err(e) => println!("Термометр: ошибка - {}", e),
    }

    // Теперь выключим розетку и снова выведем отчёт
    socket.turn_off()?;
    println!("\n--- Розетка выключена ---");
    println!(
        "Розетка: {}",
        if socket.is_on()? {
            "включена"
        } else {
            "выключена"
        }
    );
    println!("Мощность: {} Вт", socket.current_power()?);

    // Демонстрация обработки ошибок: пробуем получить температуру с таймаутом,
    // если имитатор не запущен.
    println!("\n--- Проверка обработки ошибок ---");
    match thermometer.get_temperature() {
        Ok(t) => println!("Температура получена: {}", t),
        Err(e) => println!("Ошибка получения температуры: {}", e),
    }

    Ok(())
}
