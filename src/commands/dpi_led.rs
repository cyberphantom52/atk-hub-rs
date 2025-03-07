use libatk_rs::prelude::*;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum LedEffectMode {
    Static = 0x1,
    Breathing = 0x2,
}

impl From<u8> for LedEffectMode {
    fn from(value: u8) -> Self {
        match value {
            0x1 => LedEffectMode::Static,
            0x2 => LedEffectMode::Breathing,
            _ => panic!("Invalid RGB lighting effect"),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum LedBreathingRate {
    Slow = 0x1,
    Medium = 0x3,
    Fast = 0x5,
}

impl From<u8> for LedBreathingRate {
    fn from(value: u8) -> Self {
        match value {
            0x1 => LedBreathingRate::Slow,
            0x3 => LedBreathingRate::Medium,
            0x5 => LedBreathingRate::Fast,
            _ => panic!("Invalid breathing speed"),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum LedBrightnessLevel {
    Low = 0x10,
    Medium = 0x80,
    High = 0xff,
}

impl From<u8> for LedBrightnessLevel {
    fn from(value: u8) -> Self {
        match value {
            0x10 => LedBrightnessLevel::Low,
            0x80 => LedBrightnessLevel::Medium,
            0xff => LedBrightnessLevel::High,
            _ => panic!("Invalid long bright brightness"),
        }
    }
}

#[derive(Command)]
pub struct DpiLedSettings;

#[command_extension]
impl Command<DpiLedSettings> {
    pub fn builder(self) -> CommandBuilder<DpiLedSettings> {
        let mut command = self;
        command.set_id(CommandId::SetEEPROM);
        command.set_eeprom_address(EEPROMAddress::DpiRgbLightingEffects);
        command.set_data_len(0x8).unwrap();

        CommandBuilder::new(command)
    }

    pub fn query() -> Self {
        let mut command = Command::default();
        command.set_id(CommandId::GetEEPROM);
        command.set_eeprom_address(EEPROMAddress::DpiRgbLightingEffects);
        command.set_data_len(0x8).unwrap();

        command
    }

    pub fn effect_mode(&self) -> LedEffectMode {
        self.data()[0x0].into()
    }

    pub fn set_effect_mode(&mut self, value: LedEffectMode) {
        self.set_data_byte_with_checksum(value as u8, 0x0).unwrap();
    }

    pub fn brightness_level(&self) -> LedBrightnessLevel {
        self.data()[0x2].into()
    }

    pub fn set_brightness_level(&mut self, value: LedBrightnessLevel) {
        self.set_data_byte_with_checksum(value as u8, 0x2).unwrap();
    }

    pub fn breathing_rate(&self) -> LedBreathingRate {
        self.data()[0x4].into()
    }

    pub fn set_breathing_rate(&mut self, value: LedBreathingRate) {
        self.set_data_byte_with_checksum(value as u8, 0x4).unwrap();
    }

    pub fn enabled(&self) -> bool {
        self.data()[0x6] == 0x1
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.set_data_byte_with_checksum(value as u8, 0x6).unwrap();
    }
}
