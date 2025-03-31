use chrono::Utc;
use serde::{Deserialize, Serialize};
use tauri::Emitter;
use tracing::Level;

use crate::APP_HANDLE;

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(rename_all = "camelCase")]
pub enum SerializableLevel {
	Debug,
	Info,
	Warn,
	Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LogEvent {
	#[serde(with = "chrono::serde::ts_milliseconds")]
	timestamp: chrono::DateTime<chrono::Utc>,
	level: SerializableLevel,
	msg: String,
}

#[inline(always)]
#[track_caller]
pub fn debug(msg: String) {
	tracing::event!(Level::DEBUG, msg);

	let _ = APP_HANDLE.get().unwrap().emit(
		"log",
		LogEvent {
			timestamp: Utc::now(),
			level: SerializableLevel::Debug,
			msg,
		},
	);
}

#[inline(always)]
#[track_caller]
pub fn warn(msg: String) {
	tracing::event!(Level::WARN, msg);

	let _ = APP_HANDLE.get().unwrap().emit(
		"log",
		LogEvent {
			timestamp: Utc::now(),
			level: SerializableLevel::Warn,
			msg,
		},
	);
}

#[inline(always)]
#[track_caller]
pub fn info(msg: String) {
	tracing::event!(Level::INFO, msg);

	let _ = APP_HANDLE.get().unwrap().emit(
		"log",
		LogEvent {
			timestamp: Utc::now(),
			level: SerializableLevel::Info,
			msg,
		},
	);
}

#[inline(always)]
#[track_caller]
pub fn error(msg: String) {
	tracing::event!(Level::ERROR, msg);

	let _ = APP_HANDLE.get().unwrap().emit(
		"log",
		LogEvent {
			timestamp: Utc::now(),
			level: SerializableLevel::Error,
			msg,
		},
	);
}
