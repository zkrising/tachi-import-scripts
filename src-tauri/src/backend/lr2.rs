use anyhow::bail;
use rusqlite::OptionalExtension;

use crate::backend::{
	batch_manual::{BMSBatchManualScore, BMSClient, BMSLamp, BMSOptionalMetrics, BMSScoreMeta},
	bms::BMSRandom,
};

use super::{
	batch_manual::{BMSBatchManual, BMSJudgements, BatchManualClasses, BatchManualMeta},
	bms::{BMSConvertResults, BMSGamemode},
	config::{LR2Config, SERVICE_NAME},
	log,
	sqlite::connect_sqlite3,
};

#[derive(Debug, Clone)]
pub struct ScoreRow {
	hash: String,
	clear: i32,
	perfect: i32,
	great: i32,
	good: i32,
	bad: i32,
	poor: i32,
	maxcombo: i32,
	minbp: i32,
	op_best: i32,
}

pub struct ChartRow {
	title: String,
	subtitle: Option<String>,
	mode: Option<BMSGamemode>,
}

pub fn convert_lr2_db(
	LR2Config {
		chart_path,
		score_path,
	}: &LR2Config,
) -> anyhow::Result<BMSConvertResults> {
	let score_db = connect_sqlite3(score_path)?;
	let chart_db = connect_sqlite3(chart_path)?;

	let mut db_scores = score_db.prepare("SELECT * FROM score WHERE complete = 1")?;

	let scores = db_scores.query_map([], |row| {
		Ok(ScoreRow {
			hash: row.get("hash")?,
			clear: row.get("clear")?,
			perfect: row.get("perfect")?,
			great: row.get("great")?,
			good: row.get("good")?,
			bad: row.get("bad")?,
			poor: row.get("poor")?,
			maxcombo: row.get("maxcombo")?,
			minbp: row.get("minbp")?,
			op_best: row.get("op_best")?,
		})
	})?;

	let mut scores_7k = vec![];
	let mut scores_14k = vec![];

	for score in scores {
		let score = match score {
			Ok(v) => v,
			Err(err) => {
				log::warn(format!("Invalid score in DB: {err}. Skipping."));
				continue;
			}
		};

		let mut chart_query =
			chart_db.prepare("SELECT title, subtitle, mode FROM song WHERE hash = ?1")?;

		let chart = chart_query
			.query_row([&score.hash], |row| {
				Ok(ChartRow {
					title: row.get("title")?,
					subtitle: row.get("subtitle")?,
					mode: match row.get::<_, i32>("mode")? {
						7 => Some(BMSGamemode::SevenKey),
						14 => Some(BMSGamemode::FourteenKey),
						_ => None,
					},
				})
			})
			.optional()?;

		let Some(chart) = chart else {
			log::warn(format!(
				"Couldn't find a matching chart for score {}",
				score.hash
			));
			continue;
		};

		let name = format!("{} {}", chart.title, chart.subtitle.unwrap_or_default());

		let Some(mode) = chart.mode else {
			log::debug(format!("Skipping unknown gamemode for {name}"));
			continue;
		};

		let mut random = None;

		match mode {
			BMSGamemode::SevenKey => {
				random = Some(match parse_random(score.op_best) {
					Some(v) => v,
					None => {
						bail!("Invalid random option.")
					}
				});
			}
			BMSGamemode::FourteenKey => {}
		}

		if score.minbp < 0 {
			log::info(format!(
				"Skipping score on {name} as it had a bp of {}. Probably autoscratch?",
				score.minbp
			));
			continue;
		}

		let score = BMSBatchManualScore {
			comment: None,
			identifier: score.hash,
			match_type: "bmsChartHash".into(),
			score: (score.perfect * 2 + score.great) as u64,
			lamp: match score.clear {
				0 => BMSLamp::NoPlay,
				1 => BMSLamp::Failed,
				2 => BMSLamp::EasyClear,
				3 => BMSLamp::Clear,
				4 => BMSLamp::HardClear,
				5 => BMSLamp::FullCombo,
				invalid => {
					log::warn(format!(
						"Invalid lamp on {name} -- got {invalid}; ignoring."
					));
					continue;
				}
			},
			time_achieved: None,
			optional: Some(BMSOptionalMetrics {
				bp: Some(score.minbp),
				max_combo: Some(score.maxcombo),
				fast: None,
				slow: None,
				gauge: None,
				gauge_history: None,
				epg: None,
				egr: None,
				egd: None,
				ebd: None,
				epr: None,
				lpg: None,
				lgr: None,
				lgd: None,
				lbd: None,
				lpr: None,
			}),
			score_meta: Some(BMSScoreMeta {
				random,
				input_device: None,
				client: Some(BMSClient::Lr2),
				gauge: None,
			}),
			judgements: Some(BMSJudgements {
				pgreat: Some(score.perfect),
				great: Some(score.great),
				good: Some(score.good),
				bad: Some(score.bad),
				poor: Some(score.poor),
			}),
		};

		match mode {
			BMSGamemode::SevenKey => {
				scores_7k.push(score);
			}
			BMSGamemode::FourteenKey => {
				scores_14k.push(score);
			}
		}
	}

	let mut ret = BMSConvertResults {
		k14: None,
		k7: None,
	};

	if !scores_7k.is_empty() {
		ret.k7 = Some(BMSBatchManual {
			classes: BatchManualClasses::default(),
			meta: BatchManualMeta {
				game: "bms".into(),
				playtype: "7K".into(),
				service: SERVICE_NAME.into(),
				version: None,
			},
			scores: scores_7k,
		});
	}

	if !scores_14k.is_empty() {
		ret.k14 = Some(BMSBatchManual {
			classes: BatchManualClasses::default(),
			meta: BatchManualMeta {
				game: "bms".into(),
				playtype: "14K".into(),
				service: SERVICE_NAME.into(),
				version: None,
			},
			scores: scores_14k,
		});
	}

	if ret.k14.is_none() && ret.k7.is_none() {
		log::warn("Converted no scores! Nothing will be uploaded.".into());
	}

	Ok(ret)
}

fn parse_random(rand: i32) -> Option<BMSRandom> {
	if rand > 100 {
		log::warn(format!("unknown play option {rand}. Skipping."));
		return None;
	}

	let tenths = (rand - (rand % 10)) / 10;

	Some(match tenths {
		0 => BMSRandom::Nonran,
		1 => BMSRandom::Mirror,
		2 => BMSRandom::Random,
		3 => BMSRandom::SRandom,
		unknown => {
			log::warn(format!("unknown play option {unknown}. Skipping."));
			return None;
		}
	})
}
