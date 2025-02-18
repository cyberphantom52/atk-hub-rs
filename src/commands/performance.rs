use atk_command::{Command, CommandId, EEPROMAddress};

#[derive(Command)]
#[base_offset(0x5)]
#[cmd_len(0x10)]
#[report_id(0x8)]
pub struct MousePerfSettings {
    raw: Vec<u8>,
}

impl std::fmt::Display for MousePerfSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Keystroke Anti-Shake Delay: {} ms | Motion Sync: {} | Close LED Time: {} ms | Angle Snapping: {} | Ripple Control: {}",
            self.stabilization_time(),
            self.motion_sync(),
            self.close_led_time(),
            self.linear_correction(),
            self.ripple_control()
        )
    }
}

impl MousePerfSettings {
    pub fn query() -> Self {
        let mut instance = Self {
            raw: vec![0u8; Self::cmd_len()],
        };

        instance.set_id(CommandId::GetEEPROM);
        instance.set_eeprom_address(EEPROMAddress::StabilizationTime);
        instance.set_valid_data_len(0xA);

        instance
    }

    pub fn stabilization_time(&self) -> u8 {
        self.raw[Self::base_offset()]
    }

    pub fn set_stabilization_time(&mut self, value: u8) {
        self.set_byte_pair(value, Self::base_offset()).unwrap();
    }

    pub fn motion_sync(&self) -> bool {
        self.raw[Self::base_offset() + 0x2] == 0x1
    }

    pub fn set_motion_sync(&mut self, value: bool) {
        let value = if value { 0x1 } else { 0x0 };
        self.set_byte_pair(value, Self::base_offset() + 0x2)
            .unwrap();
    }

    pub fn close_led_time(&self) -> u8 {
        self.raw[Self::base_offset() + 0x4]
    }

    pub fn set_close_led_time(&mut self, value: u8) {
        self.set_byte_pair(value, Self::base_offset() + 0x4)
            .unwrap();
    }

    pub fn linear_correction(&self) -> bool {
        self.raw[Self::base_offset() + 0x6] == 0x1
    }

    pub fn set_linear_correction(&mut self, value: bool) {
        let value = if value { 0x1 } else { 0x0 };
        self.set_byte_pair(value, Self::base_offset() + 0x6)
            .unwrap();
    }

    pub fn ripple_control(&self) -> bool {
        self.raw[Self::base_offset() + 0x8] == 0x1
    }

    pub fn set_ripple_control(&mut self, value: bool) {
        let value = if value { 0x1 } else { 0x0 };
        self.set_byte_pair(value, Self::base_offset() + 0x8)
            .unwrap();
    }

    pub fn try_from(data: &[u8]) -> Result<Self, String> {
        if data.len() != Self::cmd_len() {
            return Err(format!(
                "Invalid data length. Expected {}, got {}",
                Self::cmd_len(),
                data.len()
            ));
        }

        Ok(Self { raw: data.to_vec() })
    }
}

#[derive(Command)]
#[base_offset(0x5)]
#[cmd_len(0x10)]
#[report_id(0x8)]
pub struct SensorPerfSettings {
    raw: Vec<u8>,
}

impl std::fmt::Display for SensorPerfSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Move Close LED: {} | Sensor Sleep: {} | Sensor Sleep Time: {}s | Performance Mode: {} | RF TX Time: {}ms",
            self.move_close_led(),
            self.sensor_sleep(),
            self.sensor_sleep_time() * 10,
            self.performance_mode(),
            self.rf_tx_time())
    }
}

impl SensorPerfSettings {
    pub fn query() -> Self {
        let mut instance = Self {
            raw: vec![0u8; Self::cmd_len()],
        };

        instance.set_id(CommandId::GetEEPROM);
        instance.set_eeprom_address(EEPROMAddress::MoveCloseLights);
        instance.set_valid_data_len(0xA);

        instance
    }

    pub fn move_close_led(&self) -> bool {
        self.raw[Self::base_offset()] == 0x1
    }

    pub fn set_move_close_led(&mut self, value: bool) {
        let value = if value { 0x1 } else { 0x0 };
        self.set_byte_pair(value, Self::base_offset()).unwrap();
    }

    pub fn sensor_sleep(&self) -> bool {
        self.raw[Self::base_offset() + 0x2] == 0x1
    }

    pub fn set_sensor_sleep(&mut self, value: bool) {
        let value = if value { 0x1 } else { 0x0 };
        self.set_byte_pair(value, Self::base_offset() + 0x2)
            .unwrap();
    }

    pub fn sensor_sleep_time(&self) -> u8 {
        self.raw[Self::base_offset() + 0x4]
    }

    pub fn set_sensor_sleep_time(&mut self, value: u8) {
        self.set_byte_pair(value, Self::base_offset() + 0x4)
            .unwrap();
    }

    pub fn performance_mode(&self) -> bool {
        self.raw[Self::base_offset() + 0x6] == 0x1
    }

    pub fn set_performance_mode(&mut self, value: bool) {
        let value = if value { 0x1 } else { 0x0 };
        self.set_byte_pair(value, Self::base_offset() + 0x6)
            .unwrap();
    }

    pub fn rf_tx_time(&self) -> u8 {
        self.raw[Self::base_offset() + 0x8]
    }

    pub fn set_rf_tx_time(&mut self, value: u8) {
        self.set_byte_pair(value, Self::base_offset() + 0x8)
            .unwrap();
    }

    pub fn try_from(data: &[u8]) -> Result<Self, String> {
        if data.len() != Self::cmd_len() {
            return Err(format!(
                "Invalid data length. Expected {}, got {}",
                Self::cmd_len(),
                data.len()
            ));
        }

        Ok(Self { raw: data.to_vec() })
    }
}
