use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::backend::log;

use super::usc::USCPlaytype;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
	pub name: String,
	pub base_url: String,
	pub client_url: String,
	pub client_id: String,
}

impl Default for ServerConfig {
	fn default() -> Self {
		Self {
			name: "Bokutachi".into(),
			base_url: "https://boku.tachi.ac".into(),
			client_id: "CI18c4ebe4297a9e66960ad7b7bc88e91ace634ef8".into(),
			client_url: "https://boku.tachi.ac".into(),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LR2Config {
	pub score_path: PathBuf,
	pub chart_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BeatorajaConfig {
	pub score_path: PathBuf,
	pub chart_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct USCConfig {
	pub db_path: PathBuf,
	pub playtype: USCPlaytype,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TISConfig {
	#[serde(default = "ServerConfig::default")]
	pub server: ServerConfig,

	#[serde(rename = "lr2DB")]
	pub lr2: Option<LR2Config>,
	#[serde(rename = "beatorajaDB")]
	pub beatoraja_db: Option<BeatorajaConfig>,
	#[serde(rename = "uscDB")]
	pub usc_db: Option<USCConfig>,

	#[serde(rename = "authToken")]
	pub auth_token: Option<String>,
	pub warning: String,

	pub staging: Option<bool>,
}

impl Default for TISConfig {
	fn default() -> Self {
		Self {
			server: Default::default(),
			lr2: Default::default(),
			beatoraja_db: Default::default(),
			usc_db: Default::default(),
			auth_token: Default::default(),
			warning: "THIS FILE WILL CONTAIN AN API AUTH KEY. DON'T SEND IT TO ANYONE!".into(),
			staging: Default::default(),
		}
	}
}

impl TISConfig {
	const PATH: &str = "tis-config.json";

	pub fn save(&self) {
		let res = fs::write(
			Self::PATH,
			serde_json::to_string_pretty(self).expect("must ser"),
		);

		if let Err(_err) = res {
			log::warn("Failed to save config file.".to_string());
		}
	}

	pub fn load() -> TISConfig {
		if fs::exists(Self::PATH).is_ok_and(|e| !e) {
			let default = Self::default();
			default.save();
			return default;
		}

		match fs::read(Self::PATH) {
			Ok(v) => match serde_json::from_slice(&v) {
				Ok(v) => v,
				Err(err) => {
					log::warn(format!("Failed to read config file. {err:?}"));

					TISConfig::default()
				}
			},
			Err(err) => {
				log::warn(format!("Failed to read config file. {err:?}"));

				TISConfig::default()
			}
		}
	}
}

pub const SERVICE_NAME: &str = "TIS. v2.1.0";
