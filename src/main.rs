mod commands;
mod manager;
mod types;

pub mod proto {
    tonic::include_proto!("_");
    mod atk_hub {
        tonic::include_proto!("atk_hub");
    }
    pub use atk_hub::*;
}

fn parse_field<U, T, E>(
    field: Option<U>,
    converter: impl Fn(u8) -> Result<T, E>,
    err_msg: &'static str,
) -> Result<Option<T>, tonic::Status>
where
    U: TryInto<u8>,
{
    field
        .and_then(|n| n.try_into().ok())
        .map(converter)
        .transpose()
        .map_err(|_| tonic::Status::internal(err_msg))
}

use manager::MouseManager;
use proto::{
    atk_hub_server::{AtkHub, AtkHubServer},
    LedBreathingRate, LedBrightnessLevel, LedEffectMode,
};
use tokio::sync::Mutex;
use tonic::transport::Server;

#[derive(Debug, Default)]
pub struct AtkHubService {
    manager: Mutex<MouseManager>,
}

#[tonic::async_trait]
impl AtkHub for AtkHubService {
    async fn get_battery_status(
        &self,
        _: tonic::Request<proto::Empty>,
    ) -> Result<tonic::Response<proto::BatteryStatusResponse>, tonic::Status> {
        let level = self
            .manager
            .lock()
            .await
            .battery_level()
            .map_err(|e| {
                tonic::Status::internal(format!("Failed to get battery level: {}", e.to_string()))
            })?
            .level();

        Ok(tonic::Response::new(proto::BatteryStatusResponse {
            battery_level: level as _,
        }))
    }

    async fn get_led_effect(
        &self,
        _: tonic::Request<proto::Empty>,
    ) -> Result<tonic::Response<proto::LedEffectResponse>, tonic::Status> {
        let settings = self
            .manager
            .lock()
            .await
            .profile()
            .dpi_led_settings()
            .clone();

        Ok(tonic::Response::new(settings.into()))
    }

    async fn set_led_effect(
        &self,
        request: tonic::Request<proto::LedEffectRequest>,
    ) -> Result<tonic::Response<proto::LedEffectResponse>, tonic::Status> {
        let input = request.get_ref();

        let mode = parse_field(
            input.mode,
            LedEffectMode::try_from,
            "Failed to parse mode: Must be Static or Breathing",
        )?;

        let brightness = parse_field(
            input.brightness,
            LedBrightnessLevel::try_from,
            "Failed to parse brightness: Must be Low, Medium or High",
        )?;

        let rate = parse_field(
            input.rate,
            LedBreathingRate::try_from,
            "Failed to parse rate: Must be Slow, Medium or Fast",
        )?;

        self.manager
            .lock()
            .await
            .set_dpi_led_settings(input.enable, mode, brightness, rate)
            .map_err(|e| {
                tonic::Status::internal(format!("Failed to set led effect: {}", e.to_string()))
            })?;

        let settings = self
            .manager
            .lock()
            .await
            .profile()
            .dpi_led_settings()
            .clone();

        Ok(tonic::Response::new(settings.into()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let atk_hub = AtkHubService::default();

    Server::builder()
        .add_service(AtkHubServer::new(atk_hub))
        .serve(addr)
        .await?;

    Ok(())
}
