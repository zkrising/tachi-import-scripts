//! Extremely minimal implementation of batch manual in rust.
//!
//! Note that absolutely no attempt at "proper typing" is undertook here. Basically everything
//! is stringly typed.
//!
//! This is because I have approximately 2 hours to finish this code before I fall asleep.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::bms::BMSRandom;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct BatchManualClasses(pub HashMap<String, String>);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchManualMeta {
	pub game: String,
	pub playtype: String,
	pub service: String,
	pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BMSBatchManual {
	pub meta: BatchManualMeta,
	pub scores: Vec<BMSBatchManualScore>,
	pub classes: BatchManualClasses,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct USCBatchManual {
	pub meta: BatchManualMeta,
	pub scores: Vec<USCBatchManualScore>,
	pub classes: BatchManualClasses,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BMSLamp {
	#[serde(rename = "NO PLAY")]
	NoPlay,
	#[serde(rename = "FAILED")]
	Failed,
	#[serde(rename = "ASSIST CLEAR")]
	AssistClear,
	#[serde(rename = "EASY CLEAR")]
	EasyClear,
	#[serde(rename = "CLEAR")]
	Clear,
	#[serde(rename = "HARD CLEAR")]
	HardClear,
	#[serde(rename = "EX HARD CLEAR")]
	ExHardClear,
	#[serde(rename = "FULL COMBO")]
	FullCombo,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BMSOptionalMetrics {
	pub fast: Option<i32>,
	pub slow: Option<i32>,
	pub max_combo: Option<i32>,
	pub bp: Option<i32>,
	pub gauge: Option<f64>,
	pub gauge_history: Option<Vec<f64>>,
	pub epg: Option<i32>,
	pub egr: Option<i32>,
	pub egd: Option<i32>,
	pub ebd: Option<i32>,
	pub epr: Option<i32>,
	pub lpg: Option<i32>,
	pub lgr: Option<i32>,
	pub lgd: Option<i32>,
	pub lbd: Option<i32>,
	pub lpr: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BMSInputDevice {
	#[serde(rename = "BM_CONTROLLER")]
	BmController,
	#[serde(rename = "KEYBOARD")]
	Keyboard,
	#[serde(rename = "MIDI")]
	Midi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BMSGauge {
	#[serde(rename = "EASY")]
	Easy,
	#[serde(rename = "NORMAL")]
	Normal,
	#[serde(rename = "HARD")]
	Hard,
	#[serde(rename = "EX-HARD")]
	ExHard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BMSClient {
	#[serde(rename = "lr2oraja")]
	Lr2oraja,
	#[serde(rename = "LR2")]
	Lr2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BMSScoreMeta {
	pub random: Option<BMSRandom>,
	pub input_device: Option<BMSInputDevice>,
	pub client: Option<BMSClient>,
	pub gauge: Option<BMSGauge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BMSBatchManualScore {
	pub identifier: String,
	pub match_type: String,
	pub score: u64,
	pub lamp: BMSLamp,
	pub comment: Option<String>,
	pub time_achieved: Option<i64>,
	pub optional: Option<BMSOptionalMetrics>,
	pub score_meta: Option<BMSScoreMeta>,
	pub judgements: Option<BMSJudgements>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BMSJudgements {
	pub pgreat: Option<i32>,
	pub great: Option<i32>,
	pub good: Option<i32>,
	pub bad: Option<i32>,
	pub poor: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USCJudgements {
	pub critical: Option<i32>,
	pub near: Option<i32>,
	pub miss: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum USCNoteMod {
	#[serde(rename = "MIR-RAN")]
	MirRan,
	#[serde(rename = "MIRROR")]
	Mirror,
	#[serde(rename = "NORMAL")]
	Normal,
	#[serde(rename = "RANDOM")]
	Random,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum USCGaugeMod {
	#[serde(rename = "NORMAL")]
	Normal,
	#[serde(rename = "HARD")]
	Hard,
	#[serde(rename = "PERMISSIVE")]
	Permissive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct USCScoreMeta {
	pub note_mod: Option<USCNoteMod>,
	pub gauge_mod: Option<USCGaugeMod>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum USCLamp {
	#[serde(rename = "FAILED")]
	Failed,
	#[serde(rename = "CLEAR")]
	Clear,
	#[serde(rename = "EXCESSIVE CLEAR")]
	ExcessiveClear,
	#[serde(rename = "ULTIMATE CHAIN")]
	UltimateChain,
	#[serde(rename = "PERFECT ULTIMATE CHAIN")]
	PerfectUltimateChain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct USCOptionalMetrics {
	pub fast: Option<i32>,
	pub slow: Option<i32>,
	pub max_combo: Option<i32>,
	pub gauge: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USCBatchManualScore {
	pub identifier: String,
	pub match_type: String,
	pub score: u64,
	pub lamp: USCLamp,
	pub comment: Option<String>,
	pub time_achieved: Option<i64>,
	pub optional: Option<USCOptionalMetrics>,
	pub score_meta: Option<USCScoreMeta>,
	pub judgements: Option<USCJudgements>,
}
