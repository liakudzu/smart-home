//! Библиотека умной розетки с C ABI.
//!
//! Предоставляет функционал включения/выключения розетки и запроса мощности
//! через C-совместимый интерфейс. Все функции работают с непрозрачным
//! указателем (`*mut c_void`), что является стандартной практикой для C ABI.
//!
//! # Артефакты сборки
//!
//! - `lib` — Rust-библиотека (rlib)
//! - `staticlib` — статическая библиотека (.a) с C ABI
//! - `cdylib` — динамическая библиотека (.so) с C ABI

use std::ffi::c_double;
use std::os::raw::c_void;

/// Внутреннее состояние умной розетки (приватный тип).
struct SmartSocket {
    enabled: bool,
    power_when_on: f64,
}

/// Создаёт новую розетку с указанной мощностью во включённом состоянии.
///
/// Возвращает непрозрачный указатель на созданную розетку. Уничтожить розетку
/// нужно вызовом [`smart_socket_destroy`].
///
/// # Safety
///
/// Вызывающий обязан в дальнейшем уничтожить розетку через
/// [`smart_socket_destroy`], чтобы избежать утечки памяти.
#[no_mangle]
pub extern "C" fn smart_socket_create(power_when_on: c_double) -> *mut c_void {
    let socket = SmartSocket {
        enabled: false,
        power_when_on,
    };
    Box::into_raw(Box::new(socket)) as *mut c_void
}

/// Уничтожает розетку и освобождает память.
///
/// # Safety
///
/// `socket` должен быть ненулевым указателем, полученным из
/// [`smart_socket_create`]. После вызова указатель становится
/// недействительным.
#[no_mangle]
pub unsafe extern "C" fn smart_socket_destroy(socket: *mut c_void) {
    if !socket.is_null() {
        drop(Box::from_raw(socket.cast::<SmartSocket>()));
    }
}

/// Включает розетку.
///
/// # Safety
///
/// `socket` должен быть ненулевым указателем, полученным из
/// [`smart_socket_create`].
#[no_mangle]
pub unsafe extern "C" fn smart_socket_turn_on(socket: *mut c_void) {
    let socket = (socket as *mut SmartSocket)
        .as_mut()
        .expect("smart_socket_turn_on: null pointer");
    socket.enabled = true;
}

/// Выключает розетку.
///
/// # Safety
///
/// `socket` должен быть ненулевым указателем, полученным из
/// [`smart_socket_create`].
#[no_mangle]
pub unsafe extern "C" fn smart_socket_turn_off(socket: *mut c_void) {
    let socket = (socket as *mut SmartSocket)
        .as_mut()
        .expect("smart_socket_turn_off: null pointer");
    socket.enabled = false;
}

/// Возвращает `true`, если розетка включена.
///
/// # Safety
///
/// `socket` должен быть ненулевым указателем, полученным из
/// [`smart_socket_create`].
#[no_mangle]
pub unsafe extern "C" fn smart_socket_is_on(socket: *const c_void) -> bool {
    let socket = (socket as *const SmartSocket)
        .as_ref()
        .expect("smart_socket_is_on: null pointer");
    socket.enabled
}

/// Возвращает текущую мощность розетки в ваттах.
///
/// Если розетка выключена, возвращает 0.0.
///
/// # Safety
///
/// `socket` должен быть ненулевым указателем, полученным из
/// [`smart_socket_create`].
#[no_mangle]
pub unsafe extern "C" fn smart_socket_current_power(socket: *const c_void) -> c_double {
    let socket = (socket as *const SmartSocket)
        .as_ref()
        .expect("smart_socket_current_power: null pointer");
    if socket.enabled {
        socket.power_when_on
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use std::os::raw::c_void;

    // В тестах используем такой же каст, как внутри библиотеки
    type SocketHandle = *mut c_void;

    extern "C" {
        fn smart_socket_create(power_when_on: f64) -> SocketHandle;
        fn smart_socket_destroy(socket: SocketHandle);
        fn smart_socket_turn_on(socket: SocketHandle);
        fn smart_socket_turn_off(socket: SocketHandle);
        fn smart_socket_is_on(socket: *const c_void) -> bool;
        fn smart_socket_current_power(socket: *const c_void) -> f64;
    }

    #[test]
    fn test_create_and_destroy() {
        let socket = unsafe { smart_socket_create(150.0) };
        assert!(!socket.is_null());
        unsafe { smart_socket_destroy(socket) };
    }

    #[test]
    fn test_turn_on_off() {
        let socket = unsafe { smart_socket_create(150.0) };
        unsafe {
            assert!(!smart_socket_is_on(socket));
            assert_eq!(smart_socket_current_power(socket), 0.0);

            smart_socket_turn_on(socket);
            assert!(smart_socket_is_on(socket));
            assert_eq!(smart_socket_current_power(socket), 150.0);

            smart_socket_turn_off(socket);
            assert!(!smart_socket_is_on(socket));
            assert_eq!(smart_socket_current_power(socket), 0.0);

            smart_socket_destroy(socket);
        }
    }

    #[test]
    fn test_multiple_sockets() {
        let s1 = unsafe { smart_socket_create(100.0) };
        let s2 = unsafe { smart_socket_create(200.0) };
        unsafe {
            smart_socket_turn_on(s1);
            assert!(smart_socket_is_on(s1));
            assert_eq!(smart_socket_current_power(s1), 100.0);

            assert!(!smart_socket_is_on(s2));
            assert_eq!(smart_socket_current_power(s2), 0.0);

            smart_socket_turn_on(s2);
            assert_eq!(smart_socket_current_power(s2), 200.0);

            smart_socket_destroy(s1);
            smart_socket_destroy(s2);
        }
    }
}
