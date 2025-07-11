use crate::ip_check::ip_result::IpCheckError::{CreateReqwestClient, JsonParse, ParseIP, Request};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::net::IpAddr;
use std::time::Duration;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum IpCheckError {
    #[default]
    No,
    JsonParse(String),
    Request(String),
    ParseIP(String),
    CreateReqwestClient,
    NotSupport,
}

pub fn json_parse_error_ip_result(provider: &str, message: &str) -> IpResult {
    IpResult {
        success: false,
        error: JsonParse(message.to_string()),
        provider: provider.to_string(),
        ip: None,
        autonomous_system: None,
        region: None,
        risk: None,
        used_time: None,
    }
}

pub fn request_error_ip_result(provider: &str, message: &str) -> IpResult {
    IpResult {
        success: false,
        error: Request(message.to_string()),
        provider: provider.to_string(),
        ip: None,
        autonomous_system: None,
        region: None,
        risk: None,
        used_time: None,
    }
}

pub fn parse_ip_error_ip_result(provider: &str, message: &str) -> IpResult {
    IpResult {
        success: false,
        error: ParseIP(message.to_string()),
        provider: provider.to_string(),
        ip: None,
        autonomous_system: None,
        region: None,
        risk: None,
        used_time: None,
    }
}

pub fn create_reqwest_client_error(provider: &str) -> IpResult {
    IpResult {
        success: false,
        error: CreateReqwestClient,
        provider: provider.to_string(),
        ip: None,
        autonomous_system: None,
        region: None,
        risk: None,
        used_time: None,
    }
}

pub fn not_support_error(provider: &str) -> IpResult {
    IpResult {
        success: false,
        error: IpCheckError::NotSupport,
        provider: provider.to_string(),
        ip: None,
        autonomous_system: None,
        region: None,
        risk: None,
        used_time: None,
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IpResult {
    pub success: bool,
    pub error: IpCheckError,
    pub provider: String,
    pub ip: Option<IpAddr>,
    pub autonomous_system: Option<AS>,
    pub region: Option<Region>,
    pub risk: Option<Risk>,
    pub used_time: Option<Duration>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AS {
    pub number: u32,
    pub name: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Region {
    pub country: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub coordinates: Option<Coordinates>,
    pub time_zone: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Coordinates {
    pub lat: String,
    pub lon: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Risk {
    pub risk: Option<u16>,
    pub tags: Option<Vec<RiskTag>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RiskTag {
    Tor,
    Proxy,
    Hosting,
    Relay,
    Mobile,
    Other(String),
}

pub trait IpResultVecExt {
    fn sort_by_name(&mut self);
}

impl IpResultVecExt for Vec<IpResult> {
    fn sort_by_name(&mut self) {
        self.sort_unstable_by(|a, b| {
            let len_a = a.ip.as_ref().map_or(0, |ip| ip.to_string().len());
            let len_b = b.ip.as_ref().map_or(0, |ip| ip.to_string().len());

            len_a
                .cmp(&len_b)
                .then_with(|| a.provider.cmp(&b.provider))
                .then_with(|| a.used_time.cmp(&b.used_time))
        });
    }
}

impl Display for IpResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.success {
            if let Some(ip) = &self.ip {
                write!(f, "{} Done: {}", self.provider, ip)
            } else {
                write!(f, "{} Done but have no IP", self.provider)
            }
        } else {
            write!(f, "{} Error: {}", self.provider, self.error)
        }
    }
}
