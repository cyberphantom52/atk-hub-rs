use crate::types::{Decaseconds, Duration, Milliseconds, Seconds};
use libatk_rs::prelude::*;

#[derive(Command, Default, Debug)]
pub struct MousePerfSettings {
    stabilization_time: Duration<Milliseconds>,
    motion_sync: bool,
    close_led_time: Duration<Decaseconds>,
    linear_correction: bool,
    ripple_control: bool,
}

impl std::fmt::Display for MousePerfSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Keystroke Anti-Shake Delay: {} | Move Syncronization: {} | Close LED Time: {} | Angle Snapping: {} | Ripple Control: {}",
            self.stabilization_time(),
            self.motion_sync(),
            self.close_led_time(),
            self.linear_correction(),
            self.ripple_control()
        )
    }
}

#[allow(dead_code)]
impl MousePerfSettings {
    pub fn set(
        &self,
        stabilization_time: Option<Duration<Milliseconds>>,
        motion_sync: Option<bool>,
        close_led_time: Option<Duration<Decaseconds>>,
        linear_correction: Option<bool>,
        ripple_control: Option<bool>,
    ) -> Self {
        MousePerfSettings {
            stabilization_time: stabilization_time.unwrap_or(self.stabilization_time),
            motion_sync: motion_sync.unwrap_or(self.motion_sync),
            close_led_time: close_led_time.unwrap_or(self.close_led_time),
            linear_correction: linear_correction.unwrap_or(self.linear_correction),
            ripple_control: ripple_control.unwrap_or(self.ripple_control),
        }
    }

    pub fn stabilization_time(&self) -> Duration<Milliseconds> {
        self.stabilization_time
    }

    pub fn motion_sync(&self) -> bool {
        self.motion_sync
    }

    pub fn close_led_time(&self) -> Duration<Seconds> {
        self.close_led_time.convert()
    }

    pub fn linear_correction(&self) -> bool {
        self.linear_correction
    }

    pub fn ripple_control(&self) -> bool {
        self.ripple_control
    }

    pub fn builder(&self) -> CommandBuilder<Self> {
        Command::<Self>::builder()
            .stabilization_time(self.stabilization_time)
            .motion_sync(self.motion_sync)
            .close_led_time(self.close_led_time)
            .linear_correction(self.linear_correction)
            .ripple_control(self.ripple_control)
    }
}

#[command_extension]
impl Command<MousePerfSettings> {
    pub fn query() -> Self {
        let mut instance = Command::default();

        instance.set_id(CommandId::GetEEPROM);
        instance.set_eeprom_address(EEPROMAddress::StabilizationTime);
        instance.set_data_len(0xA).unwrap();

        instance
    }

    pub fn builder() -> CommandBuilder<MousePerfSettings> {
        let mut command = Command::default();

        command.set_id(CommandId::SetEEPROM);
        command.set_eeprom_address(EEPROMAddress::StabilizationTime);
        command.set_data_len(0xA).unwrap();

        CommandBuilder::new(command)
    }

    pub fn config(self) -> MousePerfSettings {
        let stabilization_time = Duration::<Milliseconds>::new(self.data()[0x0] as u32);
        let motion_sync = self.data()[0x2] == 0x1;
        let close_led_time = Duration::<Decaseconds>::new(self.data()[0x4] as u32);
        let linear_correction = self.data()[0x6] == 0x1;
        let ripple_control = self.data()[0x8] == 0x1;

        MousePerfSettings {
            stabilization_time,
            motion_sync,
            close_led_time,
            linear_correction,
            ripple_control,
        }
    }

    pub fn set_stabilization_time(&mut self, value: Duration<Milliseconds>) {
        self.set_data_byte_with_checksum(value.as_unit() as u8, 0x0)
            .unwrap();
    }

    pub fn set_motion_sync(&mut self, value: bool) {
        self.set_data_byte_with_checksum(value as u8, 0x2).unwrap();
    }

    pub fn set_close_led_time(&mut self, value: Duration<Decaseconds>) {
        self.set_data_byte_with_checksum(value.as_unit() as u8, 0x4)
            .unwrap();
    }

    pub fn set_linear_correction(&mut self, value: bool) {
        self.set_data_byte_with_checksum(value as u8, 0x6).unwrap();
    }

    pub fn set_ripple_control(&mut self, value: bool) {
        self.set_data_byte_with_checksum(value as u8, 0x8).unwrap();
    }
}

#[derive(Command, Default, Debug)]
pub struct SensorPerfSettings {
    move_close_led: bool,
    sensor_sleep: bool,
    sensor_sleep_time: Duration<Decaseconds>,
    performance_mode: bool,
    rf_tx_time: Duration<Milliseconds>,
}

impl std::fmt::Display for SensorPerfSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Move Close LED: {} | Sensor Sleep: {} | Sensor Sleep Time: {} | Peformance Mode: {} | RF Tx Time: {}",
            self.move_close_led(),
            self.sensor_sleep(),
            self.sensor_sleep_time(),
            self.performance_mode(),
            self.rf_tx_time()
        )
    }
}

#[allow(dead_code)]
impl SensorPerfSettings {
    pub fn set(
        &self,
        move_close_led: Option<bool>,
        sensor_sleep: Option<bool>,
        sensor_sleep_time: Option<Duration<Decaseconds>>,
        performance_mode: Option<bool>,
        rf_tx_time: Option<Duration<Milliseconds>>,
    ) -> Self {
        SensorPerfSettings {
            move_close_led: move_close_led.unwrap_or(self.move_close_led),
            sensor_sleep: sensor_sleep.unwrap_or(self.sensor_sleep),
            sensor_sleep_time: sensor_sleep_time.unwrap_or(self.sensor_sleep_time),
            performance_mode: performance_mode.unwrap_or(self.performance_mode),
            rf_tx_time: rf_tx_time.unwrap_or(self.rf_tx_time),
        }
    }

    pub fn move_close_led(&self) -> bool {
        self.move_close_led
    }

    pub fn sensor_sleep(&self) -> bool {
        self.sensor_sleep
    }

    pub fn sensor_sleep_time(&self) -> Duration<Seconds> {
        self.sensor_sleep_time.convert()
    }

    pub fn performance_mode(&self) -> bool {
        self.performance_mode
    }

    pub fn rf_tx_time(&self) -> Duration<Milliseconds> {
        self.rf_tx_time
    }

    pub fn builder(&self) -> CommandBuilder<Self> {
        Command::<Self>::builder()
            .move_close_led(self.move_close_led)
            .sensor_sleep(self.sensor_sleep)
            .sensor_sleep_time(self.sensor_sleep_time)
            .performance_mode(self.performance_mode)
            .rf_tx_time(self.rf_tx_time)
    }
}

#[command_extension]
impl Command<SensorPerfSettings> {
    pub fn query() -> Self {
        let mut instance = Command::default();

        instance.set_id(CommandId::GetEEPROM);
        instance.set_eeprom_address(EEPROMAddress::MoveCloseLights);
        instance.set_data_len(0xA).unwrap();

        instance
    }

    pub fn builder() -> CommandBuilder<SensorPerfSettings> {
        let mut command = Command::default();

        command.set_id(CommandId::SetEEPROM);
        command.set_eeprom_address(EEPROMAddress::MoveCloseLights);
        command.set_data_len(0xA).unwrap();

        CommandBuilder::new(command)
    }

    pub fn config(self) -> SensorPerfSettings {
        let move_close_led = self.data()[0x0] == 0x1;
        let sensor_sleep = self.data()[0x2] == 0x1;
        let sensor_sleep_time = Duration::<Decaseconds>::new(self.data()[0x4] as u32);
        let performance_mode = self.data()[0x6] == 0x1;
        let rf_tx_time = Duration::<Milliseconds>::new(self.data()[0x8] as u32);

        SensorPerfSettings {
            move_close_led,
            sensor_sleep,
            sensor_sleep_time,
            performance_mode,
            rf_tx_time,
        }
    }

    pub fn set_move_close_led(&mut self, value: bool) {
        self.set_data_byte_with_checksum(value as u8, 0x0).unwrap();
    }

    pub fn set_sensor_sleep(&mut self, value: bool) {
        self.set_data_byte_with_checksum(value as u8, 0x2).unwrap();
    }

    pub fn set_sensor_sleep_time(&mut self, value: Duration<Decaseconds>) {
        self.set_data_byte_with_checksum(value.as_unit() as u8, 0x4)
            .unwrap();
    }

    pub fn set_performance_mode(&mut self, value: bool) {
        self.set_data_byte_with_checksum(value as u8, 0x6).unwrap();
    }

    pub fn set_rf_tx_time(&mut self, value: Duration<Milliseconds>) {
        self.set_data_byte_with_checksum(value.as_unit() as u8, 0x8)
            .unwrap();
    }
}
