#[derive(Clone, Debug)]
pub struct OnlineInfo {
    pub username: String,
    pub ip: String,
    pub used_flux: String,
    pub used_duration: String,
    pub balance: String,
}

pub enum Session {
    Initializing,
    Offline,
    LoggingIn { automatic: bool },
    Online(OnlineInfo),
    LoggingOut(OnlineInfo),
}

impl Session {
    pub const fn online(&self) -> Option<&OnlineInfo> {
        match self {
            Self::Online(info) | Self::LoggingOut(info) => Some(info),
            _ => None,
        }
    }

    pub const fn is_busy(&self) -> bool {
        matches!(
            self,
            Self::Initializing | Self::LoggingIn { .. } | Self::LoggingOut(_)
        )
    }
}
