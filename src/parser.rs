use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResticProfiles {
    pub profiles: BTreeMap<String, Profile>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub backup: Backup,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Backup {
    pub success: bool,
    pub time: String,
    pub error: String,
    pub stderr: String,
    pub duration: i64,
    #[serde(rename = "files_new")]
    pub files_new: i64,
    #[serde(rename = "files_changed")]
    pub files_changed: i64,
    #[serde(rename = "files_unmodified")]
    pub files_unmodified: i64,
    #[serde(rename = "dirs_new")]
    pub dirs_new: i64,
    #[serde(rename = "dirs_changed")]
    pub dirs_changed: i64,
    #[serde(rename = "dirs_unmodified")]
    pub dirs_unmodified: i64,
    #[serde(rename = "files_total")]
    pub files_total: i64,
    #[serde(rename = "bytes_added")]
    pub bytes_added: i64,
    #[serde(rename = "bytes_total")]
    pub bytes_total: i64,
}
