use crate::proto::{LedBreathingRate, LedBrightnessLevel, LedEffectMode};
use libatk_rs::prelude::*;

impl TryFrom<u8> for LedEffectMode {
    type Error = Self;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x1 => Ok(LedEffectMode::Static),
            0x2 => Ok(LedEffectMode::Breathing),
            _ => Err(LedEffectMode::Invalid),
        }
    }
}

impl TryFrom<u8> for LedBreathingRate {
    type Error = Self;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x1 => Ok(LedBreathingRate::Slow),
            0x3 => Ok(LedBreathingRate::Medium),
            0x5 => Ok(LedBreathingRate::Fast),
            _ => Err(LedBreathingRate::Invalid),
        }
    }
}

impl TryFrom<u8> for LedBrightnessLevel {
    type Error = Self;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x10 => Ok(LedBrightnessLevel::Low),
            0x80 => Ok(LedBrightnessLevel::Medium),
            0xff => Ok(LedBrightnessLevel::High),
            _ => Err(LedBrightnessLevel::Invalid),
        }
    }
}

#[derive(Command, Debug, Clone)]
pub struct DpiLedSettings {
    mode: LedEffectMode,
    brightness: LedBrightnessLevel,
    breathing_rate: LedBreathingRate,
    enabled: bool,
}

impl Into<crate::proto::LedEffectResponse> for DpiLedSettings {
    fn into(self) -> crate::proto::LedEffectResponse {
        crate::proto::LedEffectResponse {
            enabled: self.enabled,
            mode: self.mode.into(),
            brightness: self.brightness.into(),
            rate: self.breathing_rate.into(),
        }
    }
}

impl std::fmt::Display for DpiLedSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Enabled: {} | Mode: {:?} | Brightness: {:?} | Breating Rate: {:?}",
            self.enabled(),
            self.mode(),
            self.brightness(),
            self.breathing_rate()
        )
    }
}

impl Default for DpiLedSettings {
    fn default() -> Self {
        DpiLedSettings {
            mode: LedEffectMode::Static,
            brightness: LedBrightnessLevel::Medium,
            breathing_rate: LedBreathingRate::Medium,
            enabled: true,
        }
    }
}

impl DpiLedSettings {
    pub fn set(
        &self,
        enabled: Option<bool>,
        mode: Option<LedEffectMode>,
        brightness: Option<LedBrightnessLevel>,
        breathing_rate: Option<LedBreathingRate>,
    ) -> DpiLedSettings {
        DpiLedSettings {
            mode: mode.unwrap_or(self.mode),
            brightness: brightness.unwrap_or(self.brightness),
            breathing_rate: breathing_rate.unwrap_or(self.breathing_rate),
            enabled: enabled.unwrap_or(self.enabled),
        }
    }

    pub fn mode(&self) -> LedEffectMode {
        self.mode
    }

    pub fn brightness(&self) -> LedBrightnessLevel {
        self.brightness
    }

    pub fn breathing_rate(&self) -> LedBreathingRate {
        self.breathing_rate
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn builder(&self) -> CommandBuilder<DpiLedSettings> {
        Command::builder()
            .breathing_rate(self.breathing_rate())
            .brightness_level(self.brightness())
            .effect_mode(self.mode())
            .enabled(self.enabled())
    }
}

#[command_extension]
impl Command<DpiLedSettings> {
    pub fn builder() -> CommandBuilder<DpiLedSettings> {
        let mut command = Command::default();
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

    pub fn config(self) -> DpiLedSettings {
        DpiLedSettings {
            mode: self.data()[0x0].try_into().unwrap(),
            brightness: self.data()[0x2].try_into().unwrap(),
            breathing_rate: self.data()[0x4].try_into().unwrap(),
            enabled: self.data()[0x6] == 0x1,
        }
    }

    pub fn set_effect_mode(&mut self, value: LedEffectMode) {
        self.set_data_byte_with_checksum(value as u8, 0x0).unwrap();
    }

    pub fn set_brightness_level(&mut self, value: LedBrightnessLevel) {
        self.set_data_byte_with_checksum(value as u8, 0x2).unwrap();
    }

    pub fn set_breathing_rate(&mut self, value: LedBreathingRate) {
        self.set_data_byte_with_checksum(value as u8, 0x4).unwrap();
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.set_data_byte_with_checksum(value as u8, 0x6).unwrap();
    }
}
