use libatk_rs::prelude::*;

#[derive(Command)]
pub struct MousePerfSettings;

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

    pub fn stabilization_time(&self) -> u8 {
        self.data()[0x0]
    }

    pub fn set_stabilization_time(&mut self, value: u8) {
        self.set_data_byte_with_checksum(value, 0x0).unwrap();
    }

    pub fn motion_sync(&self) -> bool {
        self.data()[0x2] == 0x1
    }

    pub fn set_motion_sync(&mut self, value: bool) {
        self.set_data_byte_with_checksum(value as u8, 0x2).unwrap();
    }

    pub fn close_led_time(&self) -> u8 {
        self.data()[0x4]
    }

    pub fn set_close_led_time(&mut self, value: u8) {
        self.set_data_byte_with_checksum(value, 0x4).unwrap();
    }

    pub fn linear_correction(&self) -> bool {
        self.data()[0x6] == 0x1
    }

    pub fn set_linear_correction(&mut self, value: bool) {
        self.set_data_byte_with_checksum(value as u8, 0x6).unwrap();
    }

    pub fn ripple_control(&self) -> bool {
        self.data()[0x8] == 0x1
    }

    pub fn set_ripple_control(&mut self, value: bool) {
        self.set_data_byte_with_checksum(value as u8, 0x8).unwrap();
    }
}

#[derive(Command)]
pub struct SensorPerfSettings;

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

    pub fn move_close_led(&self) -> bool {
        self.data()[0x0] == 0x1
    }

    pub fn set_move_close_led(&mut self, value: bool) {
        self.set_data_byte_with_checksum(value as u8, 0x0).unwrap();
    }

    pub fn sensor_sleep(&self) -> bool {
        self.data()[0x2] == 0x1
    }

    pub fn set_sensor_sleep(&mut self, value: bool) {
        self.set_data_byte_with_checksum(value as u8, 0x2).unwrap();
    }

    pub fn sensor_sleep_time(&self) -> u8 {
        self.data()[0x4]
    }

    pub fn set_sensor_sleep_time(&mut self, value: u8) {
        self.set_data_byte_with_checksum(value, 0x4).unwrap();
    }

    pub fn performance_mode(&self) -> bool {
        self.data()[0x6] == 0x1
    }

    pub fn set_performance_mode(&mut self, value: bool) {
        self.set_data_byte_with_checksum(value as u8, 0x6).unwrap();
    }

    pub fn rf_tx_time(&self) -> u8 {
        self.data()[0x8]
    }

    pub fn set_rf_tx_time(&mut self, value: u8) {
        self.set_data_byte_with_checksum(value, 0x8).unwrap();
    }
}
