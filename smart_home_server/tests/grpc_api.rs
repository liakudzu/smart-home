//! Функциональные тесты gRPC API умного дома.

use std::net::SocketAddr;
use std::time::Duration;

use smart_home_proto::smart_home_client::SmartHomeClient;
use smart_home_proto::smart_home_server::SmartHomeServer;
use smart_home_proto::{
    AddDeviceRequest, AddRoomRequest, DeviceType, GetDeviceRequest, GetReportRequest,
    GetRoomRequest, ListDevicesRequest, ListRoomsRequest, RemoveDeviceRequest, RemoveRoomRequest,
};
use smart_home_server::SmartHomeService;
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tonic::Request;

async fn start_server() -> (SmartHomeClient<tonic::transport::Channel>, SocketAddr) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind test listener");
    let addr = listener.local_addr().expect("local addr");
    let incoming = TcpListenerStream::new(listener);

    let service = SmartHomeService::new();
    tokio::spawn(async move {
        Server::builder()
            .add_service(SmartHomeServer::new(service))
            .serve_with_incoming(incoming)
            .await
            .expect("server error");
    });

    // Ждём готовности сервера
    let mut attempts = 0;
    let channel = loop {
        match tonic::transport::Endpoint::from_shared(format!("http://{addr}"))
            .expect("endpoint")
            .connect()
            .await
        {
            Ok(channel) => break channel,
            Err(_) if attempts < 50 => {
                attempts += 1;
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
            Err(e) => panic!("не удалось подключиться к тестовому серверу: {e}"),
        }
    };

    (SmartHomeClient::new(channel), addr)
}

#[tokio::test]
async fn rooms_crud_and_list() {
    let (mut client, _) = start_server().await;

    let listed = client
        .list_rooms(Request::new(ListRoomsRequest {}))
        .await
        .expect("list rooms")
        .into_inner();
    assert!(listed.rooms.is_empty());

    let room = client
        .add_room(Request::new(AddRoomRequest {
            name: "Кухня".to_string(),
        }))
        .await
        .expect("add room")
        .into_inner();
    assert_eq!(room.name, "Кухня");
    assert!(room.device_names.is_empty());

    let listed = client
        .list_rooms(Request::new(ListRoomsRequest {}))
        .await
        .unwrap()
        .into_inner();
    assert_eq!(listed.rooms.len(), 1);
    assert_eq!(listed.rooms[0].name, "Кухня");

    let got = client
        .get_room(Request::new(GetRoomRequest {
            name: "Кухня".to_string(),
        }))
        .await
        .unwrap()
        .into_inner();
    assert_eq!(got.name, "Кухня");

    client
        .remove_room(Request::new(RemoveRoomRequest {
            name: "Кухня".to_string(),
        }))
        .await
        .expect("remove room");

    let err = client
        .get_room(Request::new(GetRoomRequest {
            name: "Кухня".to_string(),
        }))
        .await
        .expect_err("room should be gone");
    assert_eq!(err.code(), tonic::Code::NotFound);
}

#[tokio::test]
async fn devices_crud_and_report() {
    let (mut client, _) = start_server().await;

    client
        .add_room(Request::new(AddRoomRequest {
            name: "Гостиная".to_string(),
        }))
        .await
        .unwrap();

    let socket = client
        .add_device(Request::new(AddDeviceRequest {
            room_name: "Гостиная".to_string(),
            device_name: "Розетка".to_string(),
            device_type: DeviceType::Socket as i32,
            temperature: 0.0,
            power_when_on: 120.0,
        }))
        .await
        .expect("add socket")
        .into_inner();
    assert_eq!(socket.name, "Розетка");
    assert_eq!(socket.device_type, DeviceType::Socket as i32);

    let thermometer = client
        .add_device(Request::new(AddDeviceRequest {
            room_name: "Гостиная".to_string(),
            device_name: "Термометр".to_string(),
            device_type: DeviceType::Thermometer as i32,
            temperature: 22.5,
            power_when_on: 0.0,
        }))
        .await
        .expect("add thermometer")
        .into_inner();
    assert_eq!(thermometer.name, "Термометр");

    let devices = client
        .list_devices(Request::new(ListDevicesRequest {
            room_name: "Гостиная".to_string(),
        }))
        .await
        .unwrap()
        .into_inner();
    assert_eq!(devices.devices.len(), 2);

    let got = client
        .get_device(Request::new(GetDeviceRequest {
            room_name: "Гостиная".to_string(),
            device_name: "Термометр".to_string(),
        }))
        .await
        .unwrap()
        .into_inner();
    match got.details {
        Some(smart_home_proto::device::Details::Thermometer(t)) => {
            assert!((t.temperature - 22.5).abs() < f64::EPSILON);
        }
        other => panic!("ожидался термометр, получено: {other:?}"),
    }

    let room = client
        .get_room(Request::new(GetRoomRequest {
            name: "Гостиная".to_string(),
        }))
        .await
        .unwrap()
        .into_inner();
    assert_eq!(room.device_names.len(), 2);

    let report = client
        .get_report(Request::new(GetReportRequest {}))
        .await
        .unwrap()
        .into_inner();
    assert!(report.report.contains("УМНЫЙ ДОМ"));
    assert!(report.report.contains("Гостиная"));
    assert!(report.report.contains("Розетка"));
    assert!(report.report.contains("Термометр"));

    client
        .remove_device(Request::new(RemoveDeviceRequest {
            room_name: "Гостиная".to_string(),
            device_name: "Розетка".to_string(),
        }))
        .await
        .unwrap();

    let devices = client
        .list_devices(Request::new(ListDevicesRequest {
            room_name: "Гостиная".to_string(),
        }))
        .await
        .unwrap()
        .into_inner();
    assert_eq!(devices.devices.len(), 1);
    assert_eq!(devices.devices[0].name, "Термометр");
}

#[tokio::test]
async fn rejects_duplicate_and_missing() {
    let (mut client, _) = start_server().await;

    client
        .add_room(Request::new(AddRoomRequest {
            name: "Спальня".to_string(),
        }))
        .await
        .unwrap();

    let dup = client
        .add_room(Request::new(AddRoomRequest {
            name: "Спальня".to_string(),
        }))
        .await
        .expect_err("duplicate room");
    assert_eq!(dup.code(), tonic::Code::AlreadyExists);

    let missing_room = client
        .add_device(Request::new(AddDeviceRequest {
            room_name: "НетТакой".to_string(),
            device_name: "x".to_string(),
            device_type: DeviceType::Socket as i32,
            temperature: 0.0,
            power_when_on: 10.0,
        }))
        .await
        .expect_err("missing room");
    assert_eq!(missing_room.code(), tonic::Code::NotFound);

    let empty = client
        .add_room(Request::new(AddRoomRequest {
            name: "   ".to_string(),
        }))
        .await
        .expect_err("empty name");
    assert_eq!(empty.code(), tonic::Code::InvalidArgument);
}
