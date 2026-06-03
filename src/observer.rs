use crate::{Room, SmartDevice};
use std::sync::{Arc, Mutex};

/// Трейт-объект для наблюдателя. Используется для динамического полиморфизма.
pub trait Observer: Send + Sync {
    fn update(&self, device_name: &str, device: &SmartDevice);
}

/// Тип, представляющий замыкание как наблюдателя.
type ClosureObserver = Box<dyn Fn(&str, &SmartDevice) + Send + Sync>;

struct ClosureObserverImpl(ClosureObserver);

impl Observer for ClosureObserverImpl {
    fn update(&self, device_name: &str, device: &SmartDevice) {
        (self.0)(device_name, device);
    }
}

/// Наблюдаемая комната. Хранит список наблюдателей и уведомляет их о добавлении устройств.
pub struct ObservableRoom {
    inner: Room,
    observers: Arc<Mutex<Vec<Box<dyn Observer>>>>,
}

impl ObservableRoom {
    /// Создаёт новую наблюдаемую комнату.
    pub fn new(devices: std::collections::HashMap<String, SmartDevice>) -> Self {
        ObservableRoom {
            inner: Room::new(devices),
            observers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Добавляет устройство в комнату и уведомляет наблюдателей.
    pub fn add_device(&mut self, name: String, device: SmartDevice) {
        self.inner.add_device(name.clone(), device.clone());
        self.notify_observers(&name, &device);
    }

    /// Удаляет устройство из комнаты.
    pub fn remove_device(&mut self, name: &str) -> Option<SmartDevice> {
        self.inner.remove_device(name)
    }

    /// Регистрирует нового наблюдателя (реализующего трейт `Observer`).
    pub fn register_observer(&mut self, observer: Box<dyn Observer>) {
        self.observers.lock().unwrap().push(observer);
    }

    /// Регистрирует замыкание в качестве наблюдателя.
    pub fn register_callback<F>(&mut self, callback: F)
    where
        F: Fn(&str, &SmartDevice) + Send + Sync + 'static,
    {
        self.register_observer(Box::new(ClosureObserverImpl(Box::new(callback))));
    }

    /// Уведомляет всех зарегистрированных наблюдателей.
    fn notify_observers(&self, device_name: &str, device: &SmartDevice) {
        let observers = self.observers.lock().unwrap();
        for observer in observers.iter() {
            observer.update(device_name, device);
        }
    }
}

// Реализация Deref для удобного доступа к методам обычной комнаты (опционально)
impl std::ops::Deref for ObservableRoom {
    type Target = Room;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for ObservableRoom {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
