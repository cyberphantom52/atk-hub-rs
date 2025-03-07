use libatk_rs::prelude::*;

#[derive(Command)]
pub struct FarDistanceMode;

#[command_extension]
impl Command<FarDistanceMode> {
    pub fn builder() -> CommandBuilder<FarDistanceMode> {
        let mut command = Command::default();

        command.set_id(CommandId::SetFarDistanceMode);

        CommandBuilder::new(command)
    }

    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::GetFarDistanceMode);

        command
    }

    pub fn far_distance_mode(&self) -> bool {
        self.data()[0x0] == 0x01
    }

    pub fn set_far_distance_mode(&mut self, mode: bool) {
        self.set_data_byte(mode as u8, 0x0)
            .expect("Failed to set far distance mode");
    }
}
