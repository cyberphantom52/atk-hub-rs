use libatk_rs::prelude::*;

#[derive(Command)]
pub struct FactoryReset;

#[command_extension]
impl Command<FactoryReset> {
    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::RestoreFactory);

        command
    }
}
