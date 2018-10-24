pub enum Snap2Msg {
    Sync,
    Reset,
    Trig,
}

impl Snap2Msg {
    pub fn msg_type_code(&self) -> u8 {
        match self {
            Snap2Msg::Sync => 0xf0,
            Snap2Msg::Reset => 0xf1,
            Snap2Msg::Trig => 0xf2,
        }
    }
}
