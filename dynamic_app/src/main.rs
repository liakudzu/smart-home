//! Приложение, загружающее библиотеку умной розетки динамически в runtime.
//!
//! Динамическая библиотека (`libsmart_socket_ffi.so`) загружается через
//! прямые FFI-вызовы `dlopen`/`dlsym` (Linux), без внешних зависимостей.
//! Функции вызываются по указателям, полученным из разделяемой библиотеки,
//! что демонстрирует полноценную динамическую линковку.

use std::ffi::{c_double, CString};
use std::os::raw::c_void;
use std::path::Path;

type CreateFn = unsafe extern "C" fn(c_double) -> *mut c_void;
type DestroyFn = unsafe extern "C" fn(*mut c_void);
type TurnOnFn = unsafe extern "C" fn(*mut c_void);
type TurnOffFn = unsafe extern "C" fn(*mut c_void);
type IsOnFn = unsafe extern "C" fn(*const c_void) -> bool;
type PowerFn = unsafe extern "C" fn(*const c_void) -> c_double;

fn main() {
    // Ищем динамическую библиотеку
    let lib_path = find_library();
    println!("Loading dynamic library from: {}", lib_path.display());

    unsafe {
        let lib_handle = open_lib(&lib_path).expect("Failed to load dynamic library");

        // Загружаем указатели на функции
        let create: CreateFn = std::mem::transmute(
            find_sym(lib_handle, "smart_socket_create").expect("symbol not found"),
        );
        let destroy: DestroyFn = std::mem::transmute(
            find_sym(lib_handle, "smart_socket_destroy").expect("symbol not found"),
        );
        let turn_on: TurnOnFn = std::mem::transmute(
            find_sym(lib_handle, "smart_socket_turn_on").expect("symbol not found"),
        );
        let turn_off: TurnOffFn = std::mem::transmute(
            find_sym(lib_handle, "smart_socket_turn_off").expect("symbol not found"),
        );
        let is_on: IsOnFn = std::mem::transmute(
            find_sym(lib_handle, "smart_socket_is_on").expect("symbol not found"),
        );
        let power: PowerFn = std::mem::transmute(
            find_sym(lib_handle, "smart_socket_current_power").expect("symbol not found"),
        );

        // Создаём розетку
        let socket = create(200.0);
        println!("Smart Socket created (dynamic linking)");

        println!("Initial state: is_on = {}", is_on(socket));
        println!("Initial power: {} W", power(socket));

        // Включаем
        turn_on(socket);
        println!("Turned ON:  is_on = {}", is_on(socket));
        println!("            power = {} W", power(socket));

        // Выключаем
        turn_off(socket);
        println!("Turned OFF: is_on = {}", is_on(socket));
        println!("            power = {} W", power(socket));

        // Уничтожаем
        destroy(socket);
        println!("Smart Socket destroyed");

        close_lib(lib_handle);
    }
}

// ---- FFI к libdl ----

type DlHandle = *mut c_void;

const RTLD_LAZY: std::ffi::c_int = 1;

#[link(name = "dl")]
extern "C" {
    fn dlopen(filename: *const std::ffi::c_char, flags: std::ffi::c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const std::ffi::c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> std::ffi::c_int;
}

unsafe fn open_lib(path: &Path) -> Option<DlHandle> {
    let c_str = CString::new(path.to_str()?).ok()?;
    let handle = dlopen(c_str.as_ptr(), RTLD_LAZY);
    if handle.is_null() {
        None
    } else {
        Some(handle)
    }
}

unsafe fn find_sym(handle: DlHandle, symbol: &str) -> Option<*mut c_void> {
    let c_str = CString::new(symbol).ok()?;
    let ptr = dlsym(handle, c_str.as_ptr());
    if ptr.is_null() {
        None
    } else {
        Some(ptr)
    }
}

unsafe fn close_lib(handle: DlHandle) {
    dlclose(handle);
}

// ---- Поиск библиотеки ----

/// Ищет динамическую библиотеку smart_socket_ffi в стандартных местах.
fn find_library() -> std::path::PathBuf {
    // Возможные пути для libsmart_socket_ffi.so
    let candidates = [
        // Рабочая директория
        "libsmart_socket_ffi.so".into(),
        // Рядом с бинарём (target/debug/ или target/release/)
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.join("libsmart_socket_ffi.so")))
            .unwrap_or_default(),
        // Относительно корня проекта (при запуске из корня workspace)
        "target/debug/libsmart_socket_ffi.so".into(),
        "target/release/libsmart_socket_ffi.so".into(),
        // Относительно dynamic_app (если запуск из поддиректории)
        "../target/debug/libsmart_socket_ffi.so".into(),
        "../target/release/libsmart_socket_ffi.so".into(),
    ];

    for path in &candidates {
        if path.exists() {
            return path.clone();
        }
    }

    // Если ничего не нашли — используем путь по умолчанию
    "libsmart_socket_ffi.so".into()
}
