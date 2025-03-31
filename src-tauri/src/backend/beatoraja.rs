use bitflags::bitflags;
use rusqlite::OptionalExtension;

use crate::backend::{
	batch_manual::{BMSBatchManualScore, BMSClient, BMSLamp, BMSOptionalMetrics, BMSScoreMeta},
	bms::BMSRandom,
};

use super::{
	batch_manual::{BMSBatchManual, BMSJudgements, BatchManualClasses, BatchManualMeta},
	bms::{BMSConvertResults, BMSGamemode},
	config::{BeatorajaConfig, SERVICE_NAME},
	log,
	sqlite::connect_sqlite3,
};

#[derive(Debug, Clone)]
pub struct ScoreRow {
	sha256: String,
	clear: i32,
	epg: i32,
	egr: i32,
	egd: i32,
	ebd: i32,
	epr: i32,
	ems: i32,
	lpg: i32,
	lgr: i32,
	lgd: i32,
	lbd: i32,
	lpr: i32,
	lms: i32,
	combo: i32,
	minbp: i32,
	random: i32,
	date: i32,
}

pub struct ChartRow {
	title: String,
	subtitle: String,
	feature: ChartFeatures,
	mode: Option<BMSGamemode>,
}

bitflags! {
	pub struct ChartFeatures: u32 {
		const UNDEFINEDLN     = 0b1;
		const MINENOTE       = 0b10;
		const RANDOM         = 0b100;
		const LONGNOTE       = 0b1000;
		const CHARGENOTE     = 0b10000;
		const HELLCHARGENOTE = 0b100000;
		const STOPSEQUENCE   = 0b1000000;
		const SCROLL         = 0b10000000;
	}
}

pub fn convert_beatoraja_db(
	BeatorajaConfig {
		chart_path,
		score_path,
	}: &BeatorajaConfig,
) -> anyhow::Result<BMSConvertResults> {
	let score_db = connect_sqlite3(score_path)?;
	let chart_db = connect_sqlite3(chart_path)?;

	let mut db_scores = score_db.prepare("SELECT * FROM score WHERE mode = 0")?;

	let scores = db_scores.query_map([], |row| {
		Ok(ScoreRow {
			sha256: row.get("sha256")?,
			clear: row.get("clear")?,
			epg: row.get("epg")?,
			egr: row.get("egr")?,
			egd: row.get("egd")?,
			ebd: row.get("ebd")?,
			epr: row.get("epr")?,
			ems: row.get("ems")?,
			lpg: row.get("lpg")?,
			lgr: row.get("lgr")?,
			lgd: row.get("lgd")?,
			lbd: row.get("lbd")?,
			lpr: row.get("lpr")?,
			lms: row.get("lms")?,
			combo: row.get("combo")?,
			minbp: row.get("minbp")?,
			random: row.get("random")?,
			date: row.get("date")?,
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
		let mut chart_query = chart_db.prepare(
			"
			SELECT
				title, subtitle, feature, notes, mode
			FROM
				song
			WHERE
				sha256 = ?1
		",
		)?;

		let chart = chart_query
			.query_row([&score.sha256], |row| {
				Ok(ChartRow {
					feature: ChartFeatures::from_bits_truncate(row.get::<_, i32>("feature")? as u32),
					mode: match row.get::<_, i32>("mode")? {
						7 => Some(BMSGamemode::SevenKey),
						14 => Some(BMSGamemode::FourteenKey),
						_ => None,
					},
					subtitle: row.get("subtitle")?,
					title: row.get("title")?,
				})
			})
			.optional()?;

		let Some(chart) = chart else {
			log::warn(format!(
				"Couldn't find a matching chart for score {}",
				score.sha256
			));
			continue;
		};

		let name = format!("{} {}", chart.title, chart.subtitle);

		if chart.feature.contains(ChartFeatures::RANDOM) {
			log::info(format!("Skipping {name} as it has #RANDOM declarations."));
			continue;
		}

		let Some(mode) = chart.mode else {
			log::debug(format!("Skipping unknown gamemode for {name}"));
			continue;
		};

		let mut random = None;

		match mode {
			BMSGamemode::SevenKey => {
				random = Some(match score.random {
					0 => BMSRandom::Nonran,
					1 => BMSRandom::Mirror,
					2 => BMSRandom::Random,
					3 => BMSRandom::RRandom,
					4 => BMSRandom::SRandom,
					_unknown => {
						log::warn(format!(
							"Skipping score on {name} as the random was invalid or unfair (H-Ran, Spiral, etc.)"
						));
						continue;
					}
				})
			}
			BMSGamemode::FourteenKey => {}
		}

		// Various beatoraja fuckery abound here.
		let bp = if score.minbp == i32::MAX || score.minbp < 0 {
			None
		} else {
			Some(score.minbp)
		};

		let score = BMSBatchManualScore {
			comment: None,
			identifier: score.sha256,
			match_type: "bmsChartHash".into(),
			score: ((score.lpg + score.epg) * 2 + score.egr + score.lgr) as u64,
			lamp: match score.clear {
				0 => BMSLamp::NoPlay,
				1 => BMSLamp::Failed,
				2 => BMSLamp::AssistClear,
				3 => BMSLamp::AssistClear,
				4 => BMSLamp::EasyClear,
				5 => BMSLamp::Clear,
				6 => BMSLamp::HardClear,
				7 => BMSLamp::ExHardClear,
				8..=10 => BMSLamp::FullCombo,
				invalid => {
					log::warn(format!(
						"Invalid lamp on {name} -- got {invalid}; ignoring."
					));
					continue;
				}
			},
			time_achieved: Some((score.date * 1000) as i64),
			optional: Some(BMSOptionalMetrics {
				bp,
				fast: Some(score.egr + score.egd),
				slow: Some(score.lgr + score.lgd),
				max_combo: Some(score.combo),
				gauge: None,
				gauge_history: None,
				epg: Some(score.epg),
				egr: Some(score.egr),
				egd: Some(score.egd),
				ebd: Some(score.ebd),
				epr: Some(score.epr),
				lpg: Some(score.lpg),
				lgr: Some(score.lgr),
				lgd: Some(score.lgd),
				lbd: Some(score.lbd),
				lpr: Some(score.lpr),
			}),
			score_meta: Some(BMSScoreMeta {
				random,
				input_device: None,
				client: Some(BMSClient::Lr2oraja),
				gauge: None,
			}),
			judgements: Some(BMSJudgements {
				pgreat: Some(score.epg + score.lpg),
				great: Some(score.egr + score.lgr),
				good: Some(score.egd + score.lgd),
				bad: Some(score.ebd + score.lbd),
				poor: Some(score.epr + score.lpr + score.ems + score.lms),
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
