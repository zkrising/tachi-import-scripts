use serde::{Deserialize, Serialize};

use super::batch_manual::BMSBatchManual;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BMSGamemode {
	SevenKey,
	FourteenKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BMSRandom {
	#[serde(rename = "NONRAN")]
	Nonran,
	#[serde(rename = "MIRROR")]
	Mirror,
	#[serde(rename = "R-RANDOM")]
	RRandom,
	#[serde(rename = "S-RANDOM")]
	SRandom,
	#[serde(rename = "RANDOM")]
	Random,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BMSConvertResults {
	pub k7: Option<BMSBatchManual>,
	pub k14: Option<BMSBatchManual>,
}
