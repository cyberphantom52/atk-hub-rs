use atk_command::{Command, CommandId, EEPROMAddress};

#[derive(Command)]
#[cmd_len(0x10)]
#[report_id(0x8)]
#[base_offset(0x5)]
pub struct FactoryReset {
    raw: Vec<u8>,
}

impl FactoryReset {
    pub fn query() -> Self {
        let mut command = Self {
            raw: vec![0u8; Self::cmd_len()],
        };
        command.set_id(CommandId::RestoreFactory);

        command
    }
}
