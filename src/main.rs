mod commands;
mod manager;
mod types;

pub mod proto {
    tonic::include_proto!("atk_hub");
}

use proto::Empty;
use tonic::{Request, Response, Status};

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

    async fn get_mouse_version(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<proto::MouseVersionResponse>, Status> {
        let version = self.manager.lock().await.mouse_version().map_err(|e| {
            tonic::Status::internal(format!("Failed to get mouse version: {}", e.to_string()))
        })?;

        Ok(Response::new(version.into()))
    }

    async fn get_connection_type(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<proto::ConnectionTypeResponse>, Status> {
        let conn_ty = self.manager.lock().await.connection_type().map_err(|e| {
            tonic::Status::internal(format!("Failed to get connection type: {}", e.to_string()))
        })?;

        Ok(Response::new(conn_ty.into()))
    }

    async fn set_poll_rate(
        &self,
        request: Request<proto::PollRateRequest>,
    ) -> Result<Response<proto::PollRateResponse>, Status> {
        let input = request.get_ref();
        let poll_rate = input.rate().try_into().map_err(|_| {
            tonic::Status::internal("Failed to parse poll rate: Must be 125Hz, 250Hz, 500Hz, 1000Hz, 2000Hz, 4000Hz or 8000Hz")
        })?;

        self.manager
            .lock()
            .await
            .set_poll_rate(poll_rate)
            .map_err(|e| {
                tonic::Status::internal(format!("Failed to set poll rate: {}", e.to_string()))
            })?;

        let resp = self.manager.lock().await.profile().mouse_info().poll_rate();

        Ok(Response::new(proto::PollRateResponse { rate: resp as _ }))
    }

    async fn get_poll_rate(
        &self,
        request: tonic::Request<proto::Empty>,
    ) -> std::result::Result<tonic::Response<proto::PollRateResponse>, tonic::Status> {
        let resp = self.manager.lock().await.profile().mouse_info().poll_rate();

        Ok(Response::new(proto::PollRateResponse { rate: resp as _ }))
    }

    // Performance settings
    async fn get_mouse_performance(
        &self,
        request: tonic::Request<proto::Empty>,
    ) -> std::result::Result<tonic::Response<proto::MousePerformanceResponse>, tonic::Status> {
        todo!()
    }
    async fn set_mouse_performance(
        &self,
        request: tonic::Request<proto::MousePerformanceRequest>,
    ) -> std::result::Result<tonic::Response<proto::MousePerformanceResponse>, tonic::Status> {
        todo!()
    }
    async fn get_sensor_performance(
        &self,
        request: tonic::Request<proto::Empty>,
    ) -> std::result::Result<tonic::Response<proto::SensorPerformanceResponse>, tonic::Status> {
        todo!()
    }
    async fn set_sensor_performance(
        &self,
        request: tonic::Request<proto::SensorPerformanceRequest>,
    ) -> std::result::Result<tonic::Response<proto::SensorPerformanceResponse>, tonic::Status> {
        todo!()
    }

    // Profile management
    async fn get_dpi_profiles(
        &self,
        request: tonic::Request<proto::Empty>,
    ) -> std::result::Result<tonic::Response<proto::GetDpiProfilesResponse>, tonic::Status> {
        todo!()
    }
    async fn set_dpi_profile(
        &self,
        request: tonic::Request<proto::SetDpiProfileRequest>,
    ) -> std::result::Result<tonic::Response<proto::SetDpiProfileResponse>, tonic::Status> {
        todo!()
    }
    async fn set_dpi_profile_color(
        &self,
        request: tonic::Request<proto::SetDpiProfileColorRequest>,
    ) -> std::result::Result<tonic::Response<proto::SetDpiProfileColorResponse>, tonic::Status>
    {
        todo!()
    }
    async fn new_dpi_profile(
        &self,
        request: tonic::Request<proto::NewDpiProfileRequest>,
    ) -> std::result::Result<tonic::Response<proto::NewDpiProfileResponse>, tonic::Status> {
        todo!()
    }
    async fn delete_dpi_profile(
        &self,
        request: tonic::Request<proto::DeleteDpiProfileRequest>,
    ) -> std::result::Result<tonic::Response<proto::DeleteDpiProfileResponse>, tonic::Status> {
        todo!()
    }

    // Factory reset
    async fn factory_reset(
        &self,
        request: tonic::Request<proto::Empty>,
    ) -> std::result::Result<tonic::Response<proto::FactoryResetResponse>, tonic::Status> {
        todo!()
    }

    // Far distance mode
    async fn get_far_distance_mode(
        &self,
        request: tonic::Request<proto::Empty>,
    ) -> std::result::Result<tonic::Response<proto::FarDistanceModeResponse>, tonic::Status> {
        todo!()
    }
    async fn set_far_distance_mode(
        &self,
        request: tonic::Request<proto::FarDistanceModeRequest>,
    ) -> std::result::Result<tonic::Response<proto::FarDistanceModeResponse>, tonic::Status> {
        todo!()
    }

    // Silent height
    async fn get_silent_height(
        &self,
        request: tonic::Request<proto::Empty>,
    ) -> std::result::Result<tonic::Response<proto::SilentHeightResponse>, tonic::Status> {
        todo!()
    }
    async fn set_silent_height(
        &self,
        request: tonic::Request<proto::SilentHeightRequest>,
    ) -> std::result::Result<tonic::Response<proto::SilentHeightResponse>, tonic::Status> {
        todo!()
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
