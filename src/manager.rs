use crate::commands::prelude::*;
use libatk_rs::device::Device;

struct MouseConfig {
    battery: GetBatteryStatus,
    download_data: DownloadData,
    driver_status: DriverStatus,
    cid_mid: GetMouseCidMid,
    version: GetMouseVersion,
    dpi_led: DpiLedSettings,
    far_distance: FarDistanceMode,
    mouse_info: MouseInfo,
    mouse_perf: MousePerfSettings,
    sensor_perf: SensorPerfSettings,
    silent_mode: SilentHeight,
}

struct MouseManager {
    config: MouseConfig,
    device: Device,
}
