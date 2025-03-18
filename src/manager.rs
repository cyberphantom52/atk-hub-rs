use std::cell::{Ref, RefCell};

use crate::{
    commands::prelude::*,
    types::{Decaseconds, Duration, Milliseconds},
};
use libatk_rs::prelude::*;

#[derive(Default, Debug)]
pub struct Profile {
    dpi: Vec<DpiPairSetting>,
    dpi_color: Vec<ColorPairSetting>,
    dpi_led: DpiLedSettings,
    far_distance: FarDistanceMode,
    mouse_info: MouseInfo,
    mouse_perf: MousePerfSettings,
    sensor_perf: SensorPerfSettings,
    silent_mode: SilentHeight,
}

impl Profile {
    pub fn dpi_led_settings(&self) -> &DpiLedSettings {
        &self.dpi_led
    }

    pub fn far_distance_mode(&self) -> &FarDistanceMode {
        &self.far_distance
    }

    pub fn mouse_info(&self) -> &MouseInfo {
        &self.mouse_info
    }

    pub fn mouse_performance_settings(&self) -> &MousePerfSettings {
        &self.mouse_perf
    }

    pub fn sensor_performance_settings(&self) -> &SensorPerfSettings {
        &self.sensor_perf
    }

    pub fn silent_height(&self) -> &SilentHeight {
        &self.silent_mode
    }

    pub fn dpi_profile(&self, pair: DpiPair) -> (DpiProfile, DpiProfile) {
        let dpi = &self.dpi[pair as usize];
        let color = &self.dpi_color[pair as usize];
        (
            DpiProfile::new(dpi.dpi_first(), color.color_first()),
            DpiProfile::new(dpi.dpi_second(), color.color_second()),
        )
    }
}

pub struct MouseManager {
    profile: RefCell<Profile>,
    device: Device,
}

impl MouseManager {
    pub fn new(device: Device) -> Result<Self, Box<dyn std::error::Error>> {
        let mut instance = Self {
            profile: RefCell::new(Profile::default()),
            device,
        };

        instance.load_profile()?;

        Ok(instance)
    }

    pub fn execute<T: CommandDescriptor>(
        &self,
        cmd: Command<T>,
    ) -> Result<Command<T>, Box<dyn std::error::Error>> {
        let result = self.device.execute(cmd)?;

        Ok(result)
    }

    fn load_profile(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.wait_for_mouse_online()?;

        /* TODO: Keys */

        self.profile.borrow_mut().mouse_perf = self
            .device
            .execute(Command::<MousePerfSettings>::query())?
            .config();

        self.profile.borrow_mut().sensor_perf = self
            .device
            .execute(Command::<SensorPerfSettings>::query())?
            .config();

        /* TODO: GetCurrConf */

        self.profile.borrow_mut().far_distance = self
            .device
            .execute(Command::<FarDistanceMode>::query())?
            .config();

        self.profile.borrow_mut().mouse_info =
            self.device.execute(Command::<MouseInfo>::query())?.config();

        self.profile.borrow_mut().dpi.extend([
            self.device
                .execute(Command::<DpiPairSetting>::query(DpiPair::Pair1))?
                .config(),
            self.device
                .execute(Command::<DpiPairSetting>::query(DpiPair::Pair2))?
                .config(),
            self.device
                .execute(Command::<DpiPairSetting>::query(DpiPair::Pair3))?
                .config(),
            self.device
                .execute(Command::<DpiPairSetting>::query(DpiPair::Pair4))?
                .config(),
        ]);

        self.profile.borrow_mut().dpi_color.extend([
            self.device
                .execute(Command::<ColorPairSetting>::query(DpiPair::Pair1))?
                .config(),
            self.device
                .execute(Command::<ColorPairSetting>::query(DpiPair::Pair2))?
                .config(),
            self.device
                .execute(Command::<ColorPairSetting>::query(DpiPair::Pair3))?
                .config(),
            self.device
                .execute(Command::<ColorPairSetting>::query(DpiPair::Pair4))?
                .config(),
        ]);

        self.profile.borrow_mut().silent_mode = self
            .device
            .execute(Command::<SilentHeight>::query())?
            .config();

        self.profile.borrow_mut().dpi_led = self
            .device
            .execute(Command::<DpiLedSettings>::query())?
            .config();

        Ok(())
    }

    fn wrapper<U>(
        &self,
        func: impl Fn(&Self) -> Result<U, Box<dyn std::error::Error>>,
    ) -> Result<U, Box<dyn std::error::Error>> {
        self.wait_for_mouse_online()?;
        func(self)
    }

    fn wait_for_mouse_online(&self) -> Result<(), Box<dyn std::error::Error>> {
        let cmd = Command::<GetWirelessMouseOnline>::query();
        let status = self.device.execute(cmd.clone())?.mouse_status();
        if status == MouseStatus::Dormant {
            println!("Mouse is offline. Move the mouse to wake it up.");
        }
        while self.device.execute(cmd.clone())?.mouse_status() == MouseStatus::Dormant {}

        Ok(())
    }

    pub fn profile(&self) -> Ref<Profile> {
        self.profile.borrow()
    }

    pub fn battery_level(&self) -> Result<GetBatteryStatus, Box<dyn std::error::Error>> {
        self.wrapper(|_| {
            let resp = self.device.execute(Command::<GetBatteryStatus>::query())?;

            Ok(resp.config())
        })
    }

    pub fn set_stabilization_time(
        &self,
        time: Duration<Milliseconds>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.wrapper(|_| {
            let command = self
                .profile()
                .mouse_performance_settings()
                .builder()
                .stabilization_time(time)
                .build();

            self.profile.borrow_mut().mouse_perf = self.device.execute(command)?.config();

            Ok(())
        })
    }

    pub fn set_motion_sync(&self, value: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.wrapper(|_| {
            let command = self
                .profile()
                .mouse_performance_settings()
                .builder()
                .motion_sync(value)
                .build();

            self.profile.borrow_mut().mouse_perf = self.device.execute(command)?.config();

            Ok(())
        })
    }

    pub fn close_led_time(
        &self,
        time: Duration<Decaseconds>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.wrapper(|_| {
            let command = self
                .profile()
                .mouse_performance_settings()
                .builder()
                .close_led_time(time)
                .build();

            self.profile.borrow_mut().mouse_perf = self.device.execute(command)?.config();

            Ok(())
        })
    }

    pub fn set_angle_snapping(&self, value: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.wrapper(|_| {
            let command = self
                .profile()
                .mouse_performance_settings()
                .builder()
                .linear_correction(value)
                .build();

            self.profile.borrow_mut().mouse_perf = self.device.execute(command)?.config();

            Ok(())
        })
    }

    pub fn set_ripple_control(&self, value: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.wrapper(|_| {
            let command = self
                .profile()
                .mouse_performance_settings()
                .builder()
                .ripple_control(value)
                .build();

            self.profile.borrow_mut().mouse_perf = self.device.execute(command)?.config();

            Ok(())
        })
    }
}
