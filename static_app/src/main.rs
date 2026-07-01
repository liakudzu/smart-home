//! Приложение, использующее библиотеку умной розетки со статической линковкой.
//!
//! Библиотека `smart_socket_ffi` линкуется статически (через rlib).
//! Функции библиотеки имеют C ABI и принимают/возвращают непрозрачный
//! указатель (`*mut c_void`). Вызовы обёрнуты в `unsafe`, так как работа
//! с FFI-функциями требует осторожности.

use smart_socket_ffi::{
    smart_socket_create, smart_socket_current_power, smart_socket_destroy, smart_socket_is_on,
    smart_socket_turn_off, smart_socket_turn_on,
};

fn main() {
    unsafe {
        // Создаём розетку с мощностью 150 Вт
        let socket = smart_socket_create(150.0);
        println!("Smart Socket created (static linking)");

        println!("Initial state: is_on = {}", smart_socket_is_on(socket));
        println!("Initial power: {} W", smart_socket_current_power(socket));

        // Включаем
        smart_socket_turn_on(socket);
        println!("Turned ON:  is_on = {}", smart_socket_is_on(socket));
        println!(
            "            power = {} W",
            smart_socket_current_power(socket)
        );

        // Выключаем
        smart_socket_turn_off(socket);
        println!("Turned OFF: is_on = {}", smart_socket_is_on(socket));
        println!(
            "            power = {} W",
            smart_socket_current_power(socket)
        );

        // Уничтожаем
        smart_socket_destroy(socket);
        println!("Smart Socket destroyed");
    }
}
