use libatk_rs::prelude::*;

#[derive(Command)]
pub struct MousePerfSettings {
    stabilization_time: u8,
    motion_sync: bool,
    close_led_time: u8,
    linear_correction: bool,
    ripple_control: bool,
}

impl MousePerfSettings {
    pub fn stabilization_time(&self) -> u8 {
        self.stabilization_time
    }

    pub fn motion_sync(&self) -> bool {
        self.motion_sync
    }

    pub fn close_led_time(&self) -> u8 {
        self.close_led_time
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
        let stabilization_time = self.data()[0x0];
        let motion_sync = self.data()[0x2] == 0x1;
        let close_led_time = self.data()[0x4];
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

    pub fn set_stabilization_time(&mut self, value: u8) {
        self.set_data_byte_with_checksum(value, 0x0).unwrap();
    }

    pub fn set_motion_sync(&mut self, value: bool) {
        self.set_data_byte_with_checksum(value as u8, 0x2).unwrap();
    }

    pub fn set_close_led_time(&mut self, value: u8) {
        self.set_data_byte_with_checksum(value, 0x4).unwrap();
    }

    pub fn set_linear_correction(&mut self, value: bool) {
        self.set_data_byte_with_checksum(value as u8, 0x6).unwrap();
    }

    pub fn set_ripple_control(&mut self, value: bool) {
        self.set_data_byte_with_checksum(value as u8, 0x8).unwrap();
    }
}

#[derive(Command)]
pub struct SensorPerfSettings {
    move_close_led: bool,
    sensor_sleep: bool,
    sensor_sleep_time: u8,
    performance_mode: bool,
    rf_tx_time: u8,
}

impl SensorPerfSettings {
    pub fn move_close_led(&self) -> bool {
        self.move_close_led
    }

    pub fn sensor_sleep(&self) -> bool {
        self.sensor_sleep
    }

    pub fn sensor_sleep_time(&self) -> u8 {
        self.sensor_sleep_time
    }

    pub fn performance_mode(&self) -> bool {
        self.performance_mode
    }

    pub fn rf_tx_time(&self) -> u8 {
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
        let sensor_sleep_time = self.data()[0x4];
        let performance_mode = self.data()[0x6] == 0x1;
        let rf_tx_time = self.data()[0x8];

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

    pub fn set_sensor_sleep_time(&mut self, value: u8) {
        self.set_data_byte_with_checksum(value, 0x4).unwrap();
    }

    pub fn set_performance_mode(&mut self, value: bool) {
        self.set_data_byte_with_checksum(value as u8, 0x6).unwrap();
    }

    pub fn set_rf_tx_time(&mut self, value: u8) {
        self.set_data_byte_with_checksum(value, 0x8).unwrap();
    }
}
