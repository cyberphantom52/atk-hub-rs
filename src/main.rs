mod commands;
mod manager;
mod types;

pub mod proto {
    tonic::include_proto!("atk_hub");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("atk_hub");
}

use commands::prelude::Preset;
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
use types::{Duration, Milliseconds, Seconds};

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

        self.get_led_effect(Request::new(Empty {})).await
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

        self.get_poll_rate(Request::new(Empty {})).await
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
        let settings = self
            .manager
            .lock()
            .await
            .profile()
            .mouse_performance_settings()
            .into();

        Ok(Response::new(settings))
    }

    async fn get_sensor_performance(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<proto::SensorPerformanceResponse>, Status> {
        let settings = self
            .manager
            .lock()
            .await
            .profile()
            .sensor_performance_settings()
            .into();

        Ok(Response::new(settings))
    }

    async fn set_mouse_performance(
        &self,
        request: Request<proto::MousePerformanceRequest>,
    ) -> Result<Response<proto::MousePerformanceResponse>, Status> {
        let input = request.get_ref();

        let stabilization_time = input
            .stabilization_time_ms
            .map(|time| Duration::<Milliseconds>::new(time as u32));

        let close_led_time = input
            .close_led_time_sec
            .map(|time| Duration::<Seconds>::new(time as u32).convert());

        self.manager
            .lock()
            .await
            .set_mouse_performance_settings(
                stabilization_time,
                input.motion_sync,
                close_led_time,
                input.linear_correction,
                input.ripple_control,
            )
            .map_err(|e| {
                Status::internal(format!(
                    "Failed to set mouse performance settings: {}",
                    e.to_string()
                ))
            })?;

        let resp = self
            .manager
            .lock()
            .await
            .profile()
            .mouse_performance_settings()
            .into();

        Ok(Response::new(resp))
    }

    async fn set_sensor_performance(
        &self,
        request: Request<proto::SensorPerformanceRequest>,
    ) -> Result<Response<proto::SensorPerformanceResponse>, Status> {
        let input = request.get_ref();

        let sensor_sleep_time = input
            .sensor_sleep_time_sec
            .map(|time| Duration::<Seconds>::new(time as u32).convert());

        let rf_tx_time = input
            .rf_tx_time_ms
            .map(|time| Duration::<Milliseconds>::new(time as u32));

        self.manager
            .lock()
            .await
            .set_sensor_performance_settings(
                input.move_close_led,
                input.sensor_sleep,
                sensor_sleep_time,
                input.performance_mode,
                rf_tx_time,
            )
            .map_err(|e| {
                Status::internal(format!(
                    "Failed to set sensor performance settings: {}",
                    e.to_string()
                ))
            })?;

        let resp = self
            .manager
            .lock()
            .await
            .profile()
            .sensor_performance_settings()
            .into();

        Ok(Response::new(resp))
    }

    // Profile management
    async fn get_dpi_profiles(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<proto::GetProfilesResponse>, Status> {
        let profiles: Vec<proto::Profile> = self
            .manager
            .lock()
            .await
            .profile()
            .dpi_profiles()
            .iter()
            .enumerate()
            .map(|(index, profile)| proto::Profile {
                gear: 1 + index as i32,
                dpi: Some(profile.dpi().into()),
                color: Some(profile.color().into()),
            })
            .collect();

        Ok(Response::new(proto::GetProfilesResponse { profiles }))
    }

    async fn set_dpi_profile(
        &self,
        request: Request<proto::UpdateGearRequest>,
    ) -> Result<Response<proto::Profile>, Status> {
        let input = request.get_ref();
        let index = Preset::try_from(input.gear as u8).unwrap();

        if let Some(color) = input.color {
            self.manager
                .lock()
                .await
                .set_dpi_profile_color(index, color.into())
                .map_err(|e| {
                    Status::internal(format!("Failed to set DPI profile: {}", e.to_string()))
                })?;
        }

        if let Some(dpi) = input.dpi {
            self.manager
                .lock()
                .await
                .set_dpi_profile_dpi(index, dpi.into())
                .map_err(|e| {
                    Status::internal(format!("Failed to set DPI profile: {}", e.to_string()))
                })?;
        }

        let preset = self.manager.lock().await.profile().preset(index);
        let resp = proto::Profile {
            gear: index as _,
            dpi: Some(preset.dpi().into()),
            color: Some(preset.color().into()),
        };
        Ok(Response::new(resp))
    }

    async fn new_dpi_profile(
        &self,
        request: Request<proto::NewGearRequest>,
    ) -> Result<Response<proto::Profile>, Status> {
        let input = request.get_ref();
        if input.dpi.is_none() {
            return Err(Status::invalid_argument("DPI is required"));
        }
        if input.color.is_none() {
            return Err(Status::invalid_argument("Color is required"));
        }

        let dpi = input.dpi.unwrap();
        let color = input.color.unwrap();
        self.manager
            .lock()
            .await
            .new_dpi_profile(dpi.into(), color.into())
            .map_err(|e| {
                Status::internal(format!(
                    "Failed to create new DPI profile: {}",
                    e.to_string()
                ))
            })?;

        let index = Preset::try_from(
            self.manager
                .lock()
                .await
                .profile()
                .mouse_info()
                .num_profile(),
        )
        .unwrap();

        let preset = self.manager.lock().await.profile().preset(index);

        let resp = proto::Profile {
            gear: index as _,
            dpi: Some(preset.dpi().into()),
            color: Some(preset.color().into()),
        };
        Ok(Response::new(resp))
    }

    async fn delete_dpi_profile(
        &self,
        request: Request<proto::DeleteGearRequest>,
    ) -> Result<Response<Empty>, Status> {
        let input = request.get_ref();
        let preset = Preset::try_from(input.gear as u8).unwrap();

        self.manager
            .lock()
            .await
            .delete_dpi_profile(preset)
            .map_err(|e| {
                Status::internal(format!("Failed to delete DPI profile: {}", e.to_string()))
            })?;

        Ok(Response::new(proto::Empty {}))
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
        let resp = self.manager.lock().await.profile().silent_height().clone();

        Ok(Response::new(resp.into()))
    }

    async fn set_silent_height(
        &self,
        request: Request<proto::SilentHeightRequest>,
    ) -> Result<Response<proto::SilentHeightResponse>, Status> {
        let input = request.get_ref();
        let mode = input.mode().try_into().map_err(|_| {
            Status::internal("Failed to parse silent height: Must be Off, Low, Medium or High")
        })?;

        self.manager
            .lock()
            .await
            .set_silent_height(mode)
            .map_err(|e| {
                Status::internal(format!("Failed to set silent height: {}", e.to_string()))
            })?;

        self.get_silent_height(Request::new(Empty {})).await
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let atk_hub = AtkHubService::default();

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    Server::builder()
        .add_service(service)
        .add_service(AtkHubServer::new(atk_hub))
        .serve(addr)
        .await?;

    Ok(())
}
