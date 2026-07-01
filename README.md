# smart_home

Небольшой учебный Rust-проект про "умный дом".

Проект содержит:
- библиотеку с моделями устройств и дома;
- TCP-эмулятор умной розетки;
- UDP-эмулятор термометра;
- демонстрационный бинарник, который подключается к эмуляторам и показывает текущее состояние устройств;
- библиотеку умной розетки с C-совместимым ABI и приложения для статической и динамической линковки.

---

## Workspace

Проект организован как **workspace** из следующих пакетов:

| Пакет | Описание |
|---|---|
| `smart_home` | Основная библиотека с моделями `SmartHouse`, `Room`, `SmartSocket`, `SmartThermometer`, сетевыми устройствами (`network`), а также бинарники (эмуляторы, демо, `src/main.rs`) |
| `smart_socket_ffi` | Библиотека умной розетки с C-совместимым ABI (`#[no_mangle] extern "C"`) |
| `static_app` | Приложение, использующее `smart_socket_ffi` со статической линковкой |
| `dynamic_app` | Приложение, загружающее `smart_socket_ffi` динамически в runtime через `dlopen`/`dlsym` |

---

## Состав проекта

### Основной пакет `smart_home`

- `src/lib.rs` — библиотека с типами `SmartHouse`, `Room`, `SmartSocket`, `SmartThermometer`, сетевыми устройствами в модуле `network`, паттернами `observer`, `report_builder`, `smart_house_builder`.
- `src/main.rs` — локальная демонстрация базовой модели умного дома.
- `src/bin/socket_emulator.rs` — TCP-эмулятор розетки.
- `src/bin/thermometer_emulator.rs` — UDP-эмулятор термометра.
- `src/bin/demo.rs` — демонстрация работы с сетевыми устройствами.
- `thermometer.conf` — конфигурация для эмулятора термометра.

Примеры в `examples/`:
- `report_builder_demo.rs` — демонстрация `ReportBuilder`.
- `observer_demo.rs` — демонстрация паттерна наблюдателя (`ObservableRoom`).
- `builder_demo.rs` — демонстрация `SmartHouseBuilder`.

### Си-style умная розетка (`smart_socket_ffi`)

Библиотека предоставляет C-совместимый интерфейс для управления розеткой.
При сборке создаёт три артефакта:
- Rust-библиотека (rlib)
- Статическая библиотека `.a` с C ABI
- Динамическая библиотека `.so` с C ABI

**API библиотеки:**

| Функция | Описание |
|---|---|
| `smart_socket_create(power) -> *mut c_void` | Создать новую розетку |
| `smart_socket_destroy(socket)` | Уничтожить розетку |
| `smart_socket_turn_on(socket)` | Включить розетку |
| `smart_socket_turn_off(socket)` | Выключить розетку |
| `smart_socket_is_on(socket) -> bool` | Проверить, включена ли |
| `smart_socket_current_power(socket) -> f64` | Получить текущую мощность |

### Приложения

- **`static_app`** — статическая линковка: подключает `smart_socket_ffi` напрямую как Rust-зависимость. Все функции доступны на этапе компиляции.
- **`dynamic_app`** — динамическая загрузка: открывает `libsmart_socket_ffi.so` через `dlopen`, ищет функции через `dlsym` и вызывает их по указателям.

---

## Сборка всего проекта

```bash
# Собрать весь workspace
cargo build --workspace

# Проверить код
cargo check --workspace

# Запустить тесты всех пакетов
cargo test --workspace

# Проверить форматирование
cargo fmt --all -- --check

# Проверить lint
cargo clippy --workspace -- -D warnings
```

---

## Как запустить демонстрацию

### Эмуляторы и сетевая демонстрация

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
- `demo` подключается к эмуляторам, включает и выключает розетку и выводит текущее значение температуры.

Примечание по адресам и портам: файл `thermometer.conf` по умолчанию содержит адрес `127.0.0.1:8888`, а пример запуска `socket_emulator` выше использует `127.0.0.1:1234`. Убедитесь, что адреса и порты, на которых вы запускаете эмуляторы, совпадают с настройками в `thermometer.conf` и в `src/bin/demo.rs`.

### Си-style умная розетка

Статическая линковка:

```bash
cargo run --bin static_app
```

Динамическая загрузка (предварительно соберите `.so`):

```bash
cargo build -p smart_socket_ffi
cargo run --bin dynamic_app
```

---

## Запуск примеров

```bash
cargo run --example report_builder_demo
cargo run --example observer_demo
cargo run --example builder_demo
```

---

## Пример: билдер `SmartHouseBuilder`

Ниже небольшой пример использования типестейт-билдера для пошаговой сборки `SmartHouse`.

```rust
use smart_home::{
    smart_house_builder::SmartHouseBuilder,
    SmartDevice, SmartSocket, SmartThermometer,
};

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
