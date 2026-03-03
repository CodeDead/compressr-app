use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Version {
    pub semver: String,
    pub platforms: Vec<Platform>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Platform {
    pub name: String,
    pub arch: String,
    #[serde(rename = "downloadUrl")]
    pub download_url: String,
    #[serde(rename = "infoUrl")]
    pub info_url: String,
}
