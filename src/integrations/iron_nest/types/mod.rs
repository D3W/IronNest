use {
    serde::{Deserialize, Serialize},
    std::fmt,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::prelude::Type))]
#[serde(rename_all = "kebab-case")]
pub enum DeviceType {
    SmartPlug,
    SmartLight,
    RingDoorbell,
    RokuTv,
    Stoplight,

    Chromecast,
    /*
    AndroidTv,
    AppleTv,
    Chromecast,
    Kodi,
    LGTv,
    Plex,
    SamsungTv,
    Tivo,
    VizioTV,

    Spotify,
    Sonos,

    HueLight,

    Macbook,
    Minecraft,
    BrotherPrinter,
    Roomba,
    */
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SmartPlug => write!(f, "Smart Plug"),
            Self::SmartLight => write!(f, "Smart Light"),
            Self::RingDoorbell => write!(f, "Ring Doorbell"),
            Self::RokuTv => write!(f, "Roku TV"),
            Self::Stoplight => write!(f, "Stoplight"),

            Self::Chromecast => write!(f, "Chromecast"),
            /*
            Self::AndroidTv => write!(f, "AndroidTv"),
            Self::AppleTv => write!(f, "AppleTv"),
            Self::Chromecast => write!(f, "Chromecast"),
            Self::Kodi => write!(f, "Kodi"),
            Self::LGTv => write!(f, "LGTv"),
            Self::Plex => write!(f, "Plex"),
            Self::SamsungTv => write!(f, "SamsungTv"),
            Self::Tivo => write!(f, "Tivo"),
            Self::VizioTV => write!(f, "VizioTV"),
            Self::Spotify => write!(f, "Spotify"),
            Self::Sonos => write!(f, "Sonos"),
            Self::HueLight => write!(f, "HueLight"),
            Self::Macbook => write!(f, "Macbook"),
            Self::Minecraft => write!(f, "Minecraft"),
            Self::BrotherPrinter => write!(f, "BrotherPrinter"),
            Self::Roomba => write!(f, "Roomba"),
            */
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Device {
    pub id: i64,
    pub name: String,
    pub device_type: DeviceType,
    pub ip: String,
    /*
    pub port: Option<i16>,
    pub mac_address: Option<String>,
    pub service_name: Option<String>,
    */
    pub power_state: u8,
    pub battery_percentage: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct AuthState {
    pub refresh_token: String,
    pub hardware_id: String,
    pub auth_token: String,
}
