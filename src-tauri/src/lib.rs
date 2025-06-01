use serde::{Deserialize, Serialize};
use std::{
	fs,
	sync::{Arc, OnceLock},
	time::Duration,
};
use tracing::Level;

use chrono::Utc;
use parking_lot::RwLock;
use tauri::{AppHandle, Manager};

use self::backend::{
	batch_manual::USCBatchManual,
	beatoraja::convert_beatoraja_db,
	bms::BMSConvertResults,
	config::{BeatorajaConfig, LR2Config, TISConfig, USCConfig},
	log::{self, SerializableLevel},
	lr2::convert_lr2_db,
	usc::convert_usc_db,
};

mod backend;

struct State {
	pub config: Arc<RwLock<TISConfig>>,
}

#[tauri::command]
fn config(state: tauri::State<State>) -> TISConfig {
	state.config.read().clone()
}

#[tauri::command]
fn update_api_token(state: tauri::State<State>, token: String) {
	state.config.write().auth_token = Some(token);
	state.config.read().save();
}

#[tauri::command]
fn lr2_convert(state: tauri::State<State>, opts: LR2Config) -> Result<BMSConvertResults, String> {
	let conv = convert_lr2_db(&opts).map_err(|e| e.to_string())?;

	log::info("Conversion complete".to_string());

	state.config.write().lr2 = Some(opts.clone());
	state.config.read().save();

	Ok(conv)
}

#[tauri::command]
fn beatoraja_convert(
	state: tauri::State<State>,
	opts: BeatorajaConfig,
) -> Result<BMSConvertResults, String> {
	let conv = convert_beatoraja_db(&opts).map_err(|e| e.to_string())?;

	log::info("Conversion complete".to_string());

	state.config.write().beatoraja_db = Some(opts.clone());
	state.config.read().save();

	Ok(conv)
}

#[tauri::command]
fn usc_convert(state: tauri::State<State>, opts: USCConfig) -> Result<USCBatchManual, String> {
	let conv = convert_usc_db(&opts).map_err(|e| e.to_string())?;

	log::info("Conversion complete".to_string());

	state.config.write().usc_db = Some(opts.clone());
	state.config.read().save();

	Ok(conv)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ImportResponseBody {
	url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ImportResponse {
	success: bool,
	description: String,
	body: Option<ImportResponseBody>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ImportPollProgress {
	description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "importStatus")]
enum ImportPollResponseProgress {
	#[serde(rename = "completed")]
	Completed { import: serde_json::Value },
	#[serde(rename = "ongoing")]
	Ongoing { progress: ImportPollProgress },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImportPollResponseBody {
	#[serde(flatten)]
	progress: ImportPollResponseProgress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ImportPollResponse {
	success: bool,
	description: String,
	body: Option<ImportPollResponseBody>,
}

#[tauri::command]
async fn import(state: tauri::State<'_, State>, bm: serde_json::Value) -> Result<(), String> {
	let _ = fs::create_dir_all("batch-manual");

	log::info("Making import...".into());

	let game = bm
		.get("meta")
		.and_then(|e| e.get("game"))
		.and_then(|e| e.as_str().map(|e| e.to_owned()))
		.unwrap_or("invalid".to_owned());
	let playtype = bm
		.get("meta")
		.and_then(|e| e.get("playtype"))
		.and_then(|e| e.as_str().map(|e| e.to_owned()))
		.unwrap_or("invalid".to_owned());

	let filename = format!("{}-{game}-{playtype}.json", Utc::now().timestamp_millis(),);

	let _ = fs::write(
		format!("batch-manual/{filename}"),
		serde_json::to_string_pretty(&bm).expect("must ser"),
	);

	let config = state.config.read().clone();

	let Some(auth) = config.auth_token else {
		return Err("You have no auth token set up.".into());
	};

	let client = reqwest::Client::new();

	let res = client
		.post(format!(
			"{}/ir/direct-manual/import",
			config.server.base_url
		))
		.header("Authorization", format!("Bearer {auth}"))
		.header("Content-Type", "application/json")
		.header("X-User-Intent", "true")
		.header("User-Agent", "TIS/2.2.0")
		.body(serde_json::to_string(&bm).expect("must ser"))
		.send()
		.await;

	let res = res.map_err(|e| format!("Request failed: {e}"))?;

	log::info("Request OK...".into());

	let json_body: ImportResponse = res
		.json()
		.await
		.map_err(|_| "Invalid response from server.".to_string())?;

	let Some(body) = json_body.body else {
		return Err(format!(
			"Failed to submit scores. {}",
			json_body.description
		));
	};

	let mut last_desc = String::new();

	let doc = loop {
		let res = client
			.get(&body.url)
			.send()
			.await
			.map_err(|e| format!("Request failed: {e}"))?;

		let json: ImportPollResponse = res
			.json()
			.await
			.map_err(|e| format!("Invalid response from server ({e:?})."))?;

		match json.body {
			Some(body) => match body.progress {
				ImportPollResponseProgress::Completed { import } => {
					break import;
				}
				ImportPollResponseProgress::Ongoing { progress } => {
					if progress.description != last_desc {
						last_desc = progress.description;
						log::info(last_desc.clone());
					}

					tokio::time::sleep(Duration::from_secs(1)).await;
				}
			},
			None => {
				log::error(format!("Failed to process import: {}", json.description));
				return Err(format!("Failed to process import: {}", json.description));
			}
		}
	};
	log::info(format!(
		"Successfully imported scores for {game} ({playtype})"
	));

	let score_count = match doc.get("scoreIDs") {
		Some(serde_json::Value::Array(v)) => v.len(),
		_ => 0,
	};

	let failed = match doc.get("errors") {
		Some(serde_json::Value::Array(v)) => v.len(),
		_ => 0,
	};

	log::info(format!("New Scores: {score_count} | Failed {failed}",));

	Ok(())
}

#[tauri::command]
fn log(level: SerializableLevel, content: String) {
	match level {
		SerializableLevel::Debug => log::debug(content),
		SerializableLevel::Info => log::info(content),
		SerializableLevel::Warn => log::warn(content),
		SerializableLevel::Error => log::error(content),
	}
}

pub static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

pub fn run() {
	tracing_subscriber::fmt::fmt()
		.with_max_level(Level::DEBUG)
		.pretty()
		.init();

	let app = tauri::Builder::default()
		.plugin(tauri_plugin_dialog::init())
		.setup(|app| {
			app.manage(State {
				config: Arc::new(RwLock::new(TISConfig::load())),
			});

			Ok(())
		})
		.plugin(tauri_plugin_opener::init())
		.invoke_handler(tauri::generate_handler![
			config,
			update_api_token,
			usc_convert,
			lr2_convert,
			beatoraja_convert,
			import,
			log
		])
		.build(tauri::generate_context!())
		.expect("failed to boot");

	APP_HANDLE.set(app.handle().clone()).expect("must work");

	app.run(|_, _| {});
}
