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
) -> Result<Option<T>, Status>
where
    U: TryInto<u8>,
{
    field
        .and_then(|n| n.try_into().ok())
        .map(converter)
        .transpose()
        .map_err(|_| Status::internal(err_msg))
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
        _: Request<Empty>,
    ) -> Result<Response<proto::BatteryStatusResponse>, Status> {
        let level = self
            .manager
            .lock()
            .await
            .battery_level()
            .map_err(|e| {
                Status::internal(format!("Failed to get battery level: {}", e.to_string()))
            })?
            .level();

        Ok(Response::new(proto::BatteryStatusResponse {
            battery_level: level as _,
        }))
    }

    async fn get_led_effect(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<proto::LedEffectResponse>, Status> {
        let settings = self
            .manager
            .lock()
            .await
            .profile()
            .dpi_led_settings()
            .clone();

        Ok(Response::new(settings.into()))
    }

    async fn set_led_effect(
        &self,
        request: Request<proto::LedEffectRequest>,
    ) -> Result<Response<proto::LedEffectResponse>, Status> {
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
                Status::internal(format!("Failed to set led effect: {}", e.to_string()))
            })?;

        let settings = self
            .manager
            .lock()
            .await
            .profile()
            .dpi_led_settings()
            .clone();

        Ok(Response::new(settings.into()))
    }

    async fn get_mouse_version(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<proto::MouseVersionResponse>, Status> {
        let version = self.manager.lock().await.mouse_version().map_err(|e| {
            Status::internal(format!("Failed to get mouse version: {}", e.to_string()))
        })?;

        Ok(Response::new(version.into()))
    }

    async fn get_connection_type(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<proto::ConnectionTypeResponse>, Status> {
        let conn_ty = self.manager.lock().await.connection_type().map_err(|e| {
            Status::internal(format!("Failed to get connection type: {}", e.to_string()))
        })?;

        Ok(Response::new(conn_ty.into()))
    }

    async fn set_poll_rate(
        &self,
        request: Request<proto::PollRateRequest>,
    ) -> Result<Response<proto::PollRateResponse>, Status> {
        let input = request.get_ref();
        let poll_rate = input.rate().try_into().map_err(|_| {
            Status::internal("Failed to parse poll rate: Must be 125Hz, 250Hz, 500Hz, 1000Hz, 2000Hz, 4000Hz or 8000Hz")
        })?;

        self.manager
            .lock()
            .await
            .set_poll_rate(poll_rate)
            .map_err(|e| Status::internal(format!("Failed to set poll rate: {}", e.to_string())))?;

        let resp = self.manager.lock().await.profile().mouse_info().poll_rate();

        Ok(Response::new(proto::PollRateResponse { rate: resp as _ }))
    }

    async fn get_poll_rate(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<proto::PollRateResponse>, Status> {
        let resp = self.manager.lock().await.profile().mouse_info().poll_rate();

        Ok(Response::new(proto::PollRateResponse { rate: resp as _ }))
    }

    // Performance settings
    async fn get_mouse_performance(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<proto::MousePerformanceResponse>, Status> {
        todo!()
    }
    async fn set_mouse_performance(
        &self,
        request: Request<proto::MousePerformanceRequest>,
    ) -> Result<Response<proto::MousePerformanceResponse>, Status> {
        todo!()
    }
    async fn get_sensor_performance(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<proto::SensorPerformanceResponse>, Status> {
        todo!()
    }
    async fn set_sensor_performance(
        &self,
        request: Request<proto::SensorPerformanceRequest>,
    ) -> Result<Response<proto::SensorPerformanceResponse>, Status> {
        todo!()
    }

    // Profile management
    async fn get_dpi_profiles(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<proto::GetDpiProfilesResponse>, Status> {
        todo!()
    }
    async fn set_dpi_profile(
        &self,
        request: Request<proto::SetDpiProfileRequest>,
    ) -> Result<Response<proto::SetDpiProfileResponse>, Status> {
        todo!()
    }
    async fn set_dpi_profile_color(
        &self,
        request: Request<proto::SetDpiProfileColorRequest>,
    ) -> Result<Response<proto::SetDpiProfileColorResponse>, Status> {
        todo!()
    }
    async fn new_dpi_profile(
        &self,
        request: Request<proto::NewDpiProfileRequest>,
    ) -> Result<Response<proto::NewDpiProfileResponse>, Status> {
        todo!()
    }
    async fn delete_dpi_profile(
        &self,
        request: Request<proto::DeleteDpiProfileRequest>,
    ) -> Result<Response<proto::DeleteDpiProfileResponse>, Status> {
        todo!()
    }

    // Factory reset
    async fn factory_reset(&self, _: Request<Empty>) -> Result<Response<Empty>, Status> {
        self.manager
            .lock()
            .await
            .factory_reset()
            .map(|_| Response::new(Empty {}))
            .map_err(|e| Status::internal(format!("Failed to factory reset: {}", e.to_string())))
    }

    // Far distance mode
    async fn get_far_distance_mode(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<proto::FarDistanceModeResponse>, Status> {
        let resp = self
            .manager
            .lock()
            .await
            .profile()
            .far_distance_mode()
            .far_distance_mode();

        Ok(Response::new(proto::FarDistanceModeResponse {
            enabled: resp,
        }))
    }

    async fn set_far_distance_mode(
        &self,
        request: Request<proto::FarDistanceModeRequest>,
    ) -> Result<Response<proto::FarDistanceModeResponse>, Status> {
        let input = request.get_ref();

        self.manager
            .lock()
            .await
            .set_far_distance_mode(input.enabled)
            .map_err(|e| {
                Status::internal(format!(
                    "Failed to set far distance mode: {}",
                    e.to_string()
                ))
            })?;

        self.get_far_distance_mode(Request::new(Empty {})).await
    }

    // Silent height
    async fn get_silent_height(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<proto::SilentHeightResponse>, Status> {
        todo!()
    }
    async fn set_silent_height(
        &self,
        request: Request<proto::SilentHeightRequest>,
    ) -> Result<Response<proto::SilentHeightResponse>, Status> {
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
