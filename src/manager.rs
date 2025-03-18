use std::cell::{Ref, RefCell};

use crate::{
    commands::prelude::*,
    types::{Decaseconds, Duration, Milliseconds},
};
use libatk_rs::prelude::*;

#[derive(Default, Debug)]
pub struct Profile {
    dpi: [DpiPairSetting; 4],
    dpi_color: [ColorPairSetting; 4],
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

    pub fn dpi_pair_setting(&self, pair: DpiPair) -> &DpiPairSetting {
        &self.dpi[pair as usize]
    }

    pub fn dpi_profile(&self, pair: DpiPair) -> (Gear, Gear) {
        let dpi = &self.dpi[pair as usize];
        let color = &self.dpi_color[pair as usize];
        (
            Gear::new(dpi.dpi_first(), color.color_first()),
            Gear::new(dpi.dpi_second(), color.color_second()),
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

        self.profile.borrow_mut().dpi = [
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
        ];

        self.profile.borrow_mut().dpi_color = [
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
        ];

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
            let resp = Command::<GetBatteryStatus>::query().execute(&self.device)?;

            Ok(resp.config())
        })
    }

    pub fn connection_type(&self) -> Result<ConnectionType, Box<dyn std::error::Error>> {
        self.wrapper(|_| {
            let resp = Command::<DownloadData>::query().execute(&self.device)?;

            Ok(resp.config().connection_type())
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

    pub fn set_mouse_performance_settings(
        &self,
        stabilization_time: Option<Duration<Milliseconds>>,
        motion_sync: Option<bool>,
        close_led_time: Option<Duration<Decaseconds>>,
        linear_correction: Option<bool>,
        ripple_control: Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.wrapper(|_| {
            let stabilization_time = stabilization_time.unwrap_or(
                self.profile()
                    .mouse_performance_settings()
                    .stabilization_time(),
            );
            let motion_sync =
                motion_sync.unwrap_or(self.profile().mouse_performance_settings().motion_sync());
            let close_led_time = close_led_time.unwrap_or(
                self.profile()
                    .mouse_performance_settings()
                    .close_led_time()
                    .convert(),
            );
            let linear_correction = linear_correction.unwrap_or(
                self.profile()
                    .mouse_performance_settings()
                    .linear_correction(),
            );
            let ripple_control = ripple_control
                .unwrap_or(self.profile().mouse_performance_settings().ripple_control());

            let response = self
                .profile()
                .mouse_performance_settings()
                .builder()
                .stabilization_time(stabilization_time)
                .motion_sync(motion_sync)
                .linear_correction(linear_correction)
                .ripple_control(ripple_control)
                .close_led_time(close_led_time)
                .build()
                .execute(&self.device)?;

            self.profile.borrow_mut().mouse_perf = response.config();

            Ok(())
        })
    }

    pub fn set_dpi_led_enabled(&self, enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.wrapper(|_| {
            let response = self
                .profile()
                .dpi_led_settings()
                .builder()
                .enabled(enabled)
                .build()
                .execute(&self.device)?;

            self.profile.borrow_mut().dpi_led = response.config();

            Ok(())
        })
    }

    pub fn set_dpi_led_mode(&self, mode: LedEffectMode) -> Result<(), Box<dyn std::error::Error>> {
        self.wrapper(|_| {
            let response = self
                .profile()
                .dpi_led_settings()
                .builder()
                .effect_mode(mode)
                .build()
                .execute(&self.device)?;

            self.profile.borrow_mut().dpi_led = response.config();

            Ok(())
        })
    }

    pub fn set_dpi_led_brightness(
        &self,
        brightness: LedBrightnessLevel,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.wrapper(|_| {
            let response = self
                .profile()
                .dpi_led_settings()
                .builder()
                .brightness_level(brightness)
                .build()
                .execute(&self.device)?;

            self.profile.borrow_mut().dpi_led = response.config();

            Ok(())
        })
    }

    pub fn set_dpi_led_breathing_rate(
        &self,
        rate: LedBreathingRate,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.wrapper(|_| {
            let response = self
                .profile()
                .dpi_led_settings()
                .builder()
                .breathing_rate(rate)
                .build()
                .execute(&self.device)?;

            self.profile.borrow_mut().dpi_led = response.config();

            Ok(())
        })
    }

    pub fn set_dpi_led_settings(
        &self,
        enabled: Option<bool>,
        mode: Option<LedEffectMode>,
        brightness: Option<LedBrightnessLevel>,
        rate: Option<LedBreathingRate>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.wrapper(|_| {
            let enabled = enabled.unwrap_or(self.profile().dpi_led_settings().enabled());
            let mode = mode.unwrap_or(self.profile().dpi_led_settings().mode());
            let brightness = brightness.unwrap_or(self.profile().dpi_led_settings().brightness());
            let rate = rate.unwrap_or(self.profile().dpi_led_settings().breathing_rate());

            let response = self
                .profile()
                .dpi_led_settings()
                .builder()
                .enabled(enabled)
                .effect_mode(mode)
                .brightness_level(brightness)
                .breathing_rate(rate)
                .build()
                .execute(&self.device)?;

            self.profile.borrow_mut().dpi_led = response.config();

            Ok(())
        })
    }

    pub fn set_far_distance_mode(&self, mode: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.wrapper(|_| {
            let response = self
                .profile()
                .far_distance_mode()
                .builder()
                .far_distance_mode(mode)
                .build()
                .execute(&self.device)?;

            self.profile.borrow_mut().far_distance = response.config();

            Ok(())
        })
    }

    pub fn set_silent_height(
        &self,
        height: SilentHeightMode,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.wrapper(|_| {
            let response = self
                .profile()
                .silent_height()
                .builder()
                .silent_height(height)
                .build()
                .execute(&self.device)?;

            self.profile.borrow_mut().silent_mode = response.config();

            Ok(())
        })
    }

    pub fn set_move_close_led(&self, value: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.wrapper(|_| {
            let command = self
                .profile()
                .sensor_performance_settings()
                .builder()
                .move_close_led(value)
                .build();

            self.profile.borrow_mut().sensor_perf = self.device.execute(command)?.config();

            Ok(())
        })
    }

    pub fn set_sensor_sleep(&self, value: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.wrapper(|_| {
            let command = self
                .profile()
                .sensor_performance_settings()
                .builder()
                .sensor_sleep(value)
                .build();

            self.profile.borrow_mut().sensor_perf = self.device.execute(command)?.config();

            Ok(())
        })
    }

    pub fn set_performance_mode(&self, value: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.wrapper(|_| {
            let command = self
                .profile()
                .sensor_performance_settings()
                .builder()
                .performance_mode(value)
                .build();

            self.profile.borrow_mut().sensor_perf = self.device.execute(command)?.config();

            Ok(())
        })
    }

    pub fn set_sensor_performance_settings(
        &self,
        move_close_led: Option<bool>,
        sensor_sleep: Option<bool>,
        performance_mode: Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.wrapper(|_| {
            let move_close_led = move_close_led.unwrap_or(
                self.profile()
                    .sensor_performance_settings()
                    .move_close_led(),
            );
            let sensor_sleep =
                sensor_sleep.unwrap_or(self.profile().sensor_performance_settings().sensor_sleep());
            let performance_mode = performance_mode.unwrap_or(
                self.profile()
                    .sensor_performance_settings()
                    .performance_mode(),
            );

            let response = self
                .profile()
                .sensor_performance_settings()
                .builder()
                .move_close_led(move_close_led)
                .sensor_sleep(sensor_sleep)
                .performance_mode(performance_mode)
                .build()
                .execute(&self.device)?;

            self.profile.borrow_mut().sensor_perf = response.config();

            Ok(())
        })
    }

    pub fn set_dpi_profile_color(
        &self,
        profile: DpiProfile,
        color: Color,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.wrapper(|_| {
            let pair = DpiPair::try_from(profile)?;
            let which = profile as usize % 2;

            let mut builder = self.profile().dpi_color[pair as usize].builder();
            builder = if which == 0 {
                builder.color_first(color)
            } else {
                builder.color_second(color)
            };

            let response = builder.build().execute(&self.device)?;
            self.profile.borrow_mut().dpi_color[pair as usize] = response.config();

            Ok(())
        })
    }

    pub fn set_dpi_profile_dpi(
        &self,
        profile: DpiProfile,
        dpi: Dpi,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.wrapper(|_| {
            let pair = DpiPair::try_from(profile)?;
            let which = profile as usize % 2;

            let mut builder = self.profile().dpi[pair as usize].builder();
            builder = if which == 0 {
                builder.dpi_first(dpi)
            } else {
                builder.dpi_second(dpi)
            };

            let response = builder.build().execute(&self.device)?;
            self.profile.borrow_mut().dpi[pair as usize] = response.config();

            Ok(())
        })
    }

    pub fn new_dpi_profile(
        &self,
        dpi: Dpi,
        color: Color,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.wrapper(|_| {
            let num_profile = self.profile().mouse_info().num_profile();
            if num_profile >= 8 {
                return Err("Maximum number of profiles reached".into());
            }

            let response = self
                .profile()
                .mouse_info()
                .builder()
                .num_profile(num_profile + 1)
                .build()
                .execute(&self.device)?;

            self.profile.borrow_mut().mouse_info = response.config();

            let profile = DpiProfile::try_from(num_profile)?;

            self.set_dpi_profile_dpi(profile, dpi)?;
            self.set_dpi_profile_color(profile, color)?;

            Ok(())
        })
    }
}
