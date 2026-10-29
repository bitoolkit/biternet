use std::net::IpAddr;

use bon::Builder;

pub const DEFAULT_PORTAL_URL: &str = "http://10.0.0.55";
pub const DEFAULT_CAPTIVE_PORTAL_URL: &str = "http://www.bit.edu.cn";

#[derive(Clone, Debug, PartialEq, Eq, Builder)]
pub struct Config {
    #[builder(default = DEFAULT_PORTAL_URL.to_string(), into)]
    portal_url: String,
    #[builder(default = DEFAULT_CAPTIVE_PORTAL_URL.to_string(), into)]
    captive_portal_url: String,
    ip: Option<IpAddr>,
    #[builder(default)]
    dumb_terminal: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl Config {
    #[must_use]
    pub fn portal_url(&self) -> &str {
        &self.portal_url
    }

    #[must_use]
    pub fn captive_portal_url(&self) -> &str {
        &self.captive_portal_url
    }

    #[must_use]
    pub const fn ip(&self) -> Option<IpAddr> {
        self.ip
    }

    #[must_use]
    pub const fn dumb_terminal(&self) -> bool {
        self.dumb_terminal
    }
}
