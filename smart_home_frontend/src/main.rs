//! Веб-frontend умного дома: общается с backend только через gRPC.

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use smart_home_proto::smart_home_client::SmartHomeClient;
use smart_home_proto::{
    AddDeviceRequest, AddRoomRequest, DeviceType, GetDeviceRequest, GetReportRequest,
    GetRoomRequest, ListDevicesRequest, ListRoomsRequest, RemoveDeviceRequest, RemoveRoomRequest,
};
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tonic::Request;
use tower_http::services::ServeDir;
use tracing_subscriber::EnvFilter;

type GrpcClient = SmartHomeClient<Channel>;

#[derive(Clone)]
struct AppState {
    client: Arc<Mutex<GrpcClient>>,
}

#[derive(Serialize)]
struct ApiError {
    error: String,
}

#[derive(Serialize)]
struct RoomDto {
    name: String,
    device_names: Vec<String>,
}

#[derive(Serialize)]
struct DeviceDto {
    name: String,
    device_type: String,
    temperature: Option<f64>,
    enabled: Option<bool>,
    current_power: Option<f64>,
    power_when_on: Option<f64>,
}

#[derive(Deserialize)]
struct AddRoomBody {
    name: String,
}

#[derive(Deserialize)]
struct AddDeviceBody {
    name: String,
    device_type: String,
    temperature: Option<f64>,
    power_when_on: Option<f64>,
}

#[derive(Serialize)]
struct ReportDto {
    report: String,
}

fn map_status(status: tonic::Status) -> Response {
    let code = match status.code() {
        tonic::Code::NotFound => StatusCode::NOT_FOUND,
        tonic::Code::AlreadyExists => StatusCode::CONFLICT,
        tonic::Code::InvalidArgument => StatusCode::BAD_REQUEST,
        _ => StatusCode::BAD_GATEWAY,
    };
    (
        code,
        Json(ApiError {
            error: status.message().to_string(),
        }),
    )
        .into_response()
}

fn device_to_dto(device: smart_home_proto::Device) -> DeviceDto {
    let device_type =
        match DeviceType::try_from(device.device_type).unwrap_or(DeviceType::Unspecified) {
            DeviceType::Thermometer => "thermometer",
            DeviceType::Socket => "socket",
            DeviceType::Unspecified => "unknown",
        }
        .to_string();

    let mut dto = DeviceDto {
        name: device.name,
        device_type,
        temperature: None,
        enabled: None,
        current_power: None,
        power_when_on: None,
    };

    match device.details {
        Some(smart_home_proto::device::Details::Thermometer(t)) => {
            dto.temperature = Some(t.temperature);
        }
        Some(smart_home_proto::device::Details::Socket(s)) => {
            dto.enabled = Some(s.enabled);
            dto.current_power = Some(s.current_power);
            dto.power_when_on = Some(s.power_when_on);
        }
        None => {}
    }

    dto
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

async fn list_rooms(State(state): State<AppState>) -> Result<Json<Vec<RoomDto>>, Response> {
    let mut client = state.client.lock().await;
    let response = client
        .list_rooms(Request::new(ListRoomsRequest {}))
        .await
        .map_err(map_status)?;
    let rooms = response
        .into_inner()
        .rooms
        .into_iter()
        .map(|r| RoomDto {
            name: r.name,
            device_names: r.device_names,
        })
        .collect();
    Ok(Json(rooms))
}

async fn get_room(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<RoomDto>, Response> {
    let mut client = state.client.lock().await;
    let response = client
        .get_room(Request::new(GetRoomRequest { name }))
        .await
        .map_err(map_status)?;
    let room = response.into_inner();
    Ok(Json(RoomDto {
        name: room.name,
        device_names: room.device_names,
    }))
}

async fn add_room(
    State(state): State<AppState>,
    Json(body): Json<AddRoomBody>,
) -> Result<Json<RoomDto>, Response> {
    let mut client = state.client.lock().await;
    let response = client
        .add_room(Request::new(AddRoomRequest { name: body.name }))
        .await
        .map_err(map_status)?;
    let room = response.into_inner();
    Ok(Json(RoomDto {
        name: room.name,
        device_names: room.device_names,
    }))
}

async fn remove_room(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<StatusCode, Response> {
    let mut client = state.client.lock().await;
    client
        .remove_room(Request::new(RemoveRoomRequest { name }))
        .await
        .map_err(map_status)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn list_devices(
    State(state): State<AppState>,
    Path(room): Path<String>,
) -> Result<Json<Vec<DeviceDto>>, Response> {
    let mut client = state.client.lock().await;
    let response = client
        .list_devices(Request::new(ListDevicesRequest { room_name: room }))
        .await
        .map_err(map_status)?;
    Ok(Json(
        response
            .into_inner()
            .devices
            .into_iter()
            .map(device_to_dto)
            .collect(),
    ))
}

async fn get_device(
    State(state): State<AppState>,
    Path((room, device)): Path<(String, String)>,
) -> Result<Json<DeviceDto>, Response> {
    let mut client = state.client.lock().await;
    let response = client
        .get_device(Request::new(GetDeviceRequest {
            room_name: room,
            device_name: device,
        }))
        .await
        .map_err(map_status)?;
    Ok(Json(device_to_dto(response.into_inner())))
}

async fn add_device(
    State(state): State<AppState>,
    Path(room): Path<String>,
    Json(body): Json<AddDeviceBody>,
) -> Result<Json<DeviceDto>, Response> {
    let device_type = match body.device_type.as_str() {
        "thermometer" => DeviceType::Thermometer,
        "socket" => DeviceType::Socket,
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ApiError {
                    error: "device_type должен быть thermometer или socket".to_string(),
                }),
            )
                .into_response());
        }
    };

    let mut client = state.client.lock().await;
    let response = client
        .add_device(Request::new(AddDeviceRequest {
            room_name: room,
            device_name: body.name,
            device_type: device_type as i32,
            temperature: body.temperature.unwrap_or(20.0),
            power_when_on: body.power_when_on.unwrap_or(100.0),
        }))
        .await
        .map_err(map_status)?;
    Ok(Json(device_to_dto(response.into_inner())))
}

async fn remove_device(
    State(state): State<AppState>,
    Path((room, device)): Path<(String, String)>,
) -> Result<StatusCode, Response> {
    let mut client = state.client.lock().await;
    client
        .remove_device(Request::new(RemoveDeviceRequest {
            room_name: room,
            device_name: device,
        }))
        .await
        .map_err(map_status)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn get_report(State(state): State<AppState>) -> Result<Json<ReportDto>, Response> {
    let mut client = state.client.lock().await;
    let response = client
        .get_report(Request::new(GetReportRequest {}))
        .await
        .map_err(map_status)?;
    Ok(Json(ReportDto {
        report: response.into_inner().report,
    }))
}

fn static_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let grpc_addr = std::env::var("SMART_HOME_GRPC_ADDR")
        .unwrap_or_else(|_| "http://127.0.0.1:50051".to_string());
    let http_addr: SocketAddr = std::env::var("SMART_HOME_HTTP_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:3000".to_string())
        .parse()?;

    tracing::info!("connecting to gRPC backend at {grpc_addr}");
    let channel = tonic::transport::Endpoint::from_shared(grpc_addr)?
        .connect()
        .await?;
    let state = AppState {
        client: Arc::new(Mutex::new(SmartHomeClient::new(channel))),
    };

    let api = Router::new()
        .route("/rooms", get(list_rooms).post(add_room))
        .route("/rooms/{name}", get(get_room).delete(remove_room))
        .route("/rooms/{room}/devices", get(list_devices).post(add_device))
        .route(
            "/rooms/{room}/devices/{device}",
            get(get_device).delete(remove_device),
        )
        .route("/report", get(get_report))
        .with_state(state);

    let app = Router::new()
        .route("/", get(index))
        .nest("/api", api)
        .nest_service("/static", ServeDir::new(static_dir()));

    tracing::info!("frontend listening on http://{http_addr}");
    let listener = tokio::net::TcpListener::bind(http_addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
