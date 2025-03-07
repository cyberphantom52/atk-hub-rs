use libatk_rs::prelude::*;

#[derive(Command)]
pub struct GetBatteryStatus;

#[command_extension]
impl Command<GetBatteryStatus> {
    pub fn query() -> Command<GetBatteryStatus> {
        let mut command = Command::default();

        command.set_id(CommandId::GetBatteryLevel);

        command
    }

    pub fn level(&self) -> u8 {
        self.data()[0x0]
    }

    pub fn charge(&self) -> u8 {
        self.data()[0x1]
    }

    pub fn voltage(&self) -> f32 {
        self.data()[0x2] as f32 / 10f32
    }
}
