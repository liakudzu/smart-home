//! Реализация gRPC-сервиса умного дома.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use smart_home::{Report, Room, SmartDevice, SmartHouse, SmartSocket, SmartThermometer};
use smart_home_proto::smart_home_server::SmartHome;
use smart_home_proto::{
    device::Details, AddDeviceRequest, AddRoomRequest, Device, DeviceType, GetDeviceRequest,
    GetReportRequest, GetReportResponse, GetRoomRequest, ListDevicesRequest, ListDevicesResponse,
    ListRoomsRequest, ListRoomsResponse, RemoveDeviceRequest, RemoveDeviceResponse,
    RemoveRoomRequest, RemoveRoomResponse, Room as ProtoRoom, SocketDetails, ThermometerDetails,
};
use tonic::{Request, Response, Status};

/// Состояние сервера: общий умный дом под мьютексом.
#[derive(Clone)]
pub struct SmartHomeService {
    house: Arc<Mutex<SmartHouse>>,
}

impl Default for SmartHomeService {
    fn default() -> Self {
        Self::new()
    }
}

impl SmartHomeService {
    pub fn new() -> Self {
        Self {
            house: Arc::new(Mutex::new(SmartHouse::new(HashMap::new()))),
        }
    }

    fn lock_house(&self) -> std::sync::MutexGuard<'_, SmartHouse> {
        self.house
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

fn device_to_proto(name: &str, device: &SmartDevice) -> Device {
    match device {
        SmartDevice::Thermometer(t) => Device {
            name: name.to_string(),
            device_type: DeviceType::Thermometer as i32,
            details: Some(Details::Thermometer(ThermometerDetails {
                temperature: t.temperature(),
            })),
        },
        SmartDevice::Socket(s) => Device {
            name: name.to_string(),
            device_type: DeviceType::Socket as i32,
            details: Some(Details::Socket(SocketDetails {
                enabled: s.is_on(),
                current_power: s.current_power(),
                power_when_on: s.power_when_on(),
            })),
        },
    }
}

fn room_to_proto(name: &str, room: &Room) -> ProtoRoom {
    let mut device_names: Vec<String> = room.devices().map(|(n, _)| n.clone()).collect();
    device_names.sort();
    ProtoRoom {
        name: name.to_string(),
        device_names,
    }
}

#[tonic::async_trait]
impl SmartHome for SmartHomeService {
    async fn list_rooms(
        &self,
        _request: Request<ListRoomsRequest>,
    ) -> Result<Response<ListRoomsResponse>, Status> {
        let house = self.lock_house();
        let mut rooms: Vec<ProtoRoom> = house
            .rooms()
            .map(|(name, room)| room_to_proto(name, room))
            .collect();
        rooms.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(Response::new(ListRoomsResponse { rooms }))
    }

    async fn get_room(
        &self,
        request: Request<GetRoomRequest>,
    ) -> Result<Response<ProtoRoom>, Status> {
        let name = request.into_inner().name;
        let house = self.lock_house();
        let room = house
            .get_room(&name)
            .ok_or_else(|| Status::not_found(format!("комната '{name}' не найдена")))?;
        Ok(Response::new(room_to_proto(&name, room)))
    }

    async fn add_room(
        &self,
        request: Request<AddRoomRequest>,
    ) -> Result<Response<ProtoRoom>, Status> {
        let name = request.into_inner().name.trim().to_string();
        if name.is_empty() {
            return Err(Status::invalid_argument("имя комнаты не может быть пустым"));
        }

        let mut house = self.lock_house();
        if house.get_room(&name).is_some() {
            return Err(Status::already_exists(format!(
                "комната '{name}' уже существует"
            )));
        }
        house.add_room(name.clone(), Room::new(HashMap::new()));
        Ok(Response::new(ProtoRoom {
            name,
            device_names: vec![],
        }))
    }

    async fn remove_room(
        &self,
        request: Request<RemoveRoomRequest>,
    ) -> Result<Response<RemoveRoomResponse>, Status> {
        let name = request.into_inner().name;
        let mut house = self.lock_house();
        house
            .remove_room(&name)
            .ok_or_else(|| Status::not_found(format!("комната '{name}' не найдена")))?;
        Ok(Response::new(RemoveRoomResponse {}))
    }

    async fn list_devices(
        &self,
        request: Request<ListDevicesRequest>,
    ) -> Result<Response<ListDevicesResponse>, Status> {
        let room_name = request.into_inner().room_name;
        let house = self.lock_house();
        let room = house
            .get_room(&room_name)
            .ok_or_else(|| Status::not_found(format!("комната '{room_name}' не найдена")))?;

        let mut devices: Vec<Device> = room
            .devices()
            .map(|(name, device)| device_to_proto(name, device))
            .collect();
        devices.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(Response::new(ListDevicesResponse { devices }))
    }

    async fn get_device(
        &self,
        request: Request<GetDeviceRequest>,
    ) -> Result<Response<Device>, Status> {
        let req = request.into_inner();
        let house = self.lock_house();
        let device = house
            .get_device(&req.room_name, &req.device_name)
            .map_err(|e| Status::not_found(e.to_string()))?;
        Ok(Response::new(device_to_proto(&req.device_name, device)))
    }

    async fn add_device(
        &self,
        request: Request<AddDeviceRequest>,
    ) -> Result<Response<Device>, Status> {
        let req = request.into_inner();
        let room_name = req.room_name.trim().to_string();
        let device_name = req.device_name.trim().to_string();

        if room_name.is_empty() {
            return Err(Status::invalid_argument("имя комнаты не может быть пустым"));
        }
        if device_name.is_empty() {
            return Err(Status::invalid_argument(
                "имя устройства не может быть пустым",
            ));
        }

        let device_type = DeviceType::try_from(req.device_type)
            .map_err(|_| Status::invalid_argument("неизвестный тип устройства"))?;

        let smart_device = match device_type {
            DeviceType::Thermometer => SmartDevice::from(SmartThermometer::new(req.temperature)),
            DeviceType::Socket => SmartDevice::from(SmartSocket::new(req.power_when_on)),
            DeviceType::Unspecified => {
                return Err(Status::invalid_argument(
                    "необходимо указать тип устройства",
                ));
            }
        };

        let mut house = self.lock_house();
        let room = house
            .get_room_mut(&room_name)
            .ok_or_else(|| Status::not_found(format!("комната '{room_name}' не найдена")))?;

        if room.get_device(&device_name).is_some() {
            return Err(Status::already_exists(format!(
                "устройство '{device_name}' уже существует в комнате '{room_name}'"
            )));
        }

        room.add_device(device_name.clone(), smart_device.clone());
        Ok(Response::new(device_to_proto(&device_name, &smart_device)))
    }

    async fn remove_device(
        &self,
        request: Request<RemoveDeviceRequest>,
    ) -> Result<Response<RemoveDeviceResponse>, Status> {
        let req = request.into_inner();
        let mut house = self.lock_house();
        let room = house
            .get_room_mut(&req.room_name)
            .ok_or_else(|| Status::not_found(format!("комната '{}' не найдена", req.room_name)))?;

        room.remove_device(&req.device_name).ok_or_else(|| {
            Status::not_found(format!(
                "устройство '{}' не найдено в комнате '{}'",
                req.device_name, req.room_name
            ))
        })?;
        Ok(Response::new(RemoveDeviceResponse {}))
    }

    async fn get_report(
        &self,
        _request: Request<GetReportRequest>,
    ) -> Result<Response<GetReportResponse>, Status> {
        let house = self.lock_house();
        Ok(Response::new(GetReportResponse {
            report: house.report(),
        }))
    }
}
