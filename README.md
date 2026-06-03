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

Дополнительно в проекте есть примеры в `src/examples/`:
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

Терминал 3:

```bash
cargo run --bin demo
```

Ожидаемое поведение:
- `socket_emulator` слушает `127.0.0.1:1234`;
- `thermometer_emulator` отправляет температуру на адрес из `thermometer.conf`;
- `demo` подключается к обоим эмуляторам, включает и выключает розетку и выводит текущее значение температуры.

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
