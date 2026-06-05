# smart_home

Небольшой учебный Rust-проект про "умный дом".

Проект содержит:
- библиотеку с моделями устройств и дома;
- TCP-эмулятор умной розетки;
- UDP-эмулятор термометра;
- демонстрационный бинарник, который подключается к эмуляторам и показывает текущее состояние устройств.

## Состав проекта

- `src/lib.rs` - библиотека с типами `SmartHouse`, `Room`, `SmartSocket`, `SmartThermometer` и сетевыми устройствами в модуле `network`.
- `src/main.rs` - локальная демонстрация базовой модели умного дома.
- `src/bin/socket_emulator.rs` - TCP-эмулятор розетки.
- `src/bin/thermometer_emulator.rs` - UDP-эмулятор термометра.
- `src/bin/demo.rs` - демонстрация работы с сетевыми устройствами.
- `thermometer.conf` - конфигурация для эмулятора термометра.

Дополнительно в проекте есть примеры в `examples/`:
- `report_builder_demo.rs` — демонстрация `ReportBuilder`.
- `observer_demo.rs` — демонстрация паттерна наблюдателя (`ObservableRoom`).
- `builder_demo.rs` — демонстрация `SmartHouseBuilder`.

## Как запустить демонстрацию

Откройте три терминала в корне проекта.

Терминал 1:

```bash
cargo run --bin socket_emulator -- 127.0.0.1:1234 150.0
```

Терминал 2:

```bash
cargo run --bin thermometer_emulator
```

Важно: `thermometer_emulator` читает файл `thermometer.conf` из текущей рабочей директории, поэтому запускайте его из корня проекта (там же находится `thermometer.conf`). Если файла нет, создайте `thermometer.conf` с двумя строками:

```
127.0.0.1:8888
1000
```
Первая строка — адрес, на который эмулятор будет отправлять UDP-пакеты, вторая — период в миллисекундах.

Терминал 3:

```bash
cargo run --bin demo
```

Ожидаемое поведение:
- `socket_emulator` слушает `127.0.0.1:1234`;
- `thermometer_emulator` отправляет температуру на адрес из `thermometer.conf`;
- `demo` подключается к обоим эмуляторам, включает и выключает розетку и выводит текущее значение температуры.

Примечание по адресам и портам: файл `thermometer.conf` по умолчанию содержит адрес `127.0.0.1:8888`, а пример запуска `socket_emulator` выше использует `127.0.0.1:1234`. Убедитесь, что адреса и порты, на которых вы запускаете эмитаторы, совпадают с настройками в `thermometer.conf` и в `src/bin/demo.rs`.

## Проверка проекта

Проверка сборки:

```bash
cargo check
```

Проверка lint-правил:

```bash
cargo clippy
```

Проверка форматирования:

```bash
cargo fmt --check
```

## Запуск примеров

Запустить примеры можно через `cargo run --example` из корня проекта, например:

```bash
cargo run --example report_builder_demo
cargo run --example observer_demo
cargo run --example builder_demo
```

Примечание: метод `ReportBuilder::add` был переименован в `add_report`. При использовании старого имени может возникнуть несовместимость в примерах.

## Пример: билдер `SmartHouseBuilder`

Ниже небольшой пример использования типестейт‑билдера для пошаговой сборки `SmartHouse`.

```rust
use smart_home::{smart_house_builder::SmartHouseBuilder, SmartSocket, SmartThermometer, SmartDevice};

let house = SmartHouseBuilder::new()
	.add_room("Living Room")
	.add_device("Main Socket", SmartSocket::new(150.0).into())
	.add_device("Thermometer", SmartThermometer::new(22.5).into())
	.add_room("Bedroom")
	.add_device("Lamp", SmartSocket::new(75.0).into())
	.build();

println!("{}", house.report());
```

Коротко о механике:
- Билдер параметризован состоянием (`NoRoomsYet` / `HasRooms`) — это предотвращает вызов `add_device` до того, как добавлена хотя бы одна комната.
- Метод `add_room` переводит билдер в состояние `HasRooms` и возвращает билдер с возможностью добавлять устройства.
- `add_device` добавляет устройство в последнюю добавленную комнату (`current_room`).
