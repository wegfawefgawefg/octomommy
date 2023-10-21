pub enum PublicCommands {
    GiveHistory,
    RegisterSource { user_id: u32 },
}

pub enum UserCommands {
    GetHistory { client_id: u32 },
}

pub enum DebugCommands {}

pub enum OverrideCommands {
    Terminate,
}
