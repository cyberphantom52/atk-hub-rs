mod commands;
mod manager;
mod types;

pub mod proto {
    tonic::include_proto!("atk_hub");
}

use manager::MouseManager;
use proto::atk_hub_server::{AtkHub, AtkHubServer};
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
