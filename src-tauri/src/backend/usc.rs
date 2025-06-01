use anyhow::bail;
use serde::{Deserialize, Serialize};

use super::{
	batch_manual::{
		BatchManualClasses, BatchManualMeta, USCBatchManual, USCBatchManualScore, USCGaugeMod,
		USCJudgements, USCLamp, USCNoteMod, USCOptionalMetrics, USCScoreMeta,
	},
	config::{USCConfig, SERVICE_NAME},
	log,
	sqlite::connect_sqlite3,
};

#[derive(Debug, Clone)]
pub struct ScoreRow {
	score: i32,
	crit: i32,
	near: i32,
	miss: i32,
	gauge: f32,
	timestamp: i32,

	chart_hash: String,

	hit_windows: HitWindows,

	gauge_type: i32,

	auto_flags: i32,
	mirror: bool,
	random: bool,

	early: Option<i32>,
	late: Option<i32>,
	combo: Option<i32>,

	title: String,
	diff_shortname: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum USCPlaytype {
	Controller,
	Keyboard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HitWindows {
	perfect: i32,
	good: i32,
	hold: i32,
	miss: i32,
	slam: i32,
}

impl HitWindows {
	const DEFAULT: Self = Self {
		perfect: 46,
		good: 150,
		hold: 150,
		miss: 300,
		slam: 84,
	};

	const LEGACY: Self = Self {
		perfect: 46,
		good: 92,
		hold: 138,
		miss: 250,
		slam: 84,
	};

	const BUGGED: Self = Self {
		perfect: 46,
		good: 92,
		hold: 138,
		miss: 300,
		slam: 84,
	};
}

#[must_use = "actually check this you muppet"]
fn check_hit_windows(score_name: &str, windows: HitWindows) -> bool {
	if windows == HitWindows::DEFAULT {
		return true;
	}

	if windows == HitWindows::LEGACY {
		log::debug("Allowing score with legacy hit windows to be imported anyway.".to_string());
		return true;
	}

	if windows == HitWindows::BUGGED {
		log::warn(
			format!(
			"Score ({score_name}) detected with bugged hit windows! A game update has caused the new hitwindows to partially apply. YOU SHOULD GO INTO SETTINGS AND RESET YOUR HIT WINDOWS, AS YOU ARE PLAYING ON TIGHTER HIT WINDOWS THAN NORMAL!
For compatibility reasons, this score will be accepted."
		));
		return true;
	}

	false
}

fn get_lamp(score: &ScoreRow) -> USCLamp {
	if score.score == 10_000_000 {
		return USCLamp::PerfectUltimateChain;
	} else if score.miss == 0 {
		return USCLamp::UltimateChain;
	} else if score.gauge_type == 1 {
		if score.gauge > 0.0 {
			return USCLamp::ExcessiveClear;
		}
		return USCLamp::Failed;
	}

	if score.gauge > 0.7 {
		return USCLamp::Clear;
	}

	USCLamp::Failed
}

fn get_notemod(score: &ScoreRow) -> USCNoteMod {
	match (score.mirror, score.random) {
		(true, true) => USCNoteMod::MirRan,
		(true, false) => USCNoteMod::Mirror,
		(false, true) => USCNoteMod::Random,
		(false, false) => USCNoteMod::Normal,
	}
}

pub fn convert_usc_db(
	USCConfig { db_path, playtype }: &USCConfig,
) -> anyhow::Result<USCBatchManual> {
	let db = connect_sqlite3(db_path)?;

	let version: i32 =
		db.query_row("SELECT version FROM Database", [], |row| row.get("version"))?;

	if version < 19 {
		log::error(format!("The version of your maps.db is {version}, which is below the minimum of 19. Update your game. Refusing to run."));

		bail!("The version of your maps.db is {version}, which is below the minimum of 19. Update your game. Refusing to run.")
	}

	if version > 20 {
		log::error(
			format!(
			"The version of your maps.db is {version}, which is a version after what this tool supports (20). It might not be safe to convert this. Report this, and I'll update the tool to work for the later version!"
		));

		bail!(
			"The version of your maps.db is {version}, which is a version after what this tool supports (20). It might not be safe to convert this. Report this, and I'll update the tool to work for the later version!"
		);
	}

	let mut db_scores =
		db.prepare("SELECT * FROM Scores LEFT JOIN Charts ON Scores.chart_hash = Charts.hash")?;

	let scores = db_scores.query_map([], |row| {
		Ok(ScoreRow {
			score: row.get("score")?,
			crit: row.get("crit")?,
			near: row.get("near")?,
			miss: row.get("miss")?,
			gauge: row.get("gauge")?,
			timestamp: row.get("timestamp")?,
			chart_hash: row.get("chart_hash")?,
			hit_windows: HitWindows {
				perfect: row.get("window_perfect")?,
				good: row.get("window_good")?,
				hold: row.get("window_hold")?,
				miss: row.get("window_miss")?,
				slam: row.get("window_slam")?,
			},
			gauge_type: row.get("gauge_type")?,
			auto_flags: row.get("auto_flags")?,
			mirror: row.get("mirror")?,
			random: row.get("random")?,
			early: row.get("early")?,
			late: row.get("late")?,
			combo: row.get("combo")?,
			title: row.get("title")?,
			diff_shortname: row.get("diff_shortname")?,
		})
	})?;

	let mut output_scores = vec![];

	for score in scores {
		let score = match score {
			Ok(v) => v,
			Err(err) => {
				log::warn(format!("Invalid score in DB: {err}. Skipping."));
				continue;
			}
		};

		let name = format!(
			"{} [{}] ({})",
			score.title, score.diff_shortname, score.score
		);

		if !check_hit_windows(&name, score.hit_windows) {
			log::error(format!("Invalid hit windows for score {name}. Skipping."));
			continue;
		}

		if score.auto_flags != 0 {
			log::debug("Skipping".to_string());
			continue;
		}

		let score = USCBatchManualScore {
			lamp: get_lamp(&score),
			comment: None,
			identifier: score.chart_hash.clone(),
			match_type: "uscChartHash".into(),
			score: score.score as u64,
			time_achieved: Some(score.timestamp as i64 * 1000),
			optional: Some(USCOptionalMetrics {
				fast: score.early,
				slow: score.late,
				max_combo: score.combo,
				gauge: Some(score.gauge * 100.0),
			}),
			score_meta: Some(USCScoreMeta {
				note_mod: Some(get_notemod(&score)),
				gauge_mod: match score.gauge_type {
					0 => Some(USCGaugeMod::Normal),
					1 => Some(USCGaugeMod::Hard),
					unknown => {
						log::warn(format!(
							"Ignoring score on {name} as it has a gauge mod of {unknown}."
						));
						continue;
					}
				},
			}),
			judgements: Some(USCJudgements {
				critical: Some(score.crit),
				near: Some(score.near),
				miss: Some(score.miss),
			}),
		};

		output_scores.push(score);
	}

	Ok(USCBatchManual {
		classes: BatchManualClasses::default(),
		meta: BatchManualMeta {
			game: "usc".into(),
			playtype: match playtype {
				USCPlaytype::Controller => "Controller".into(),
				USCPlaytype::Keyboard => "Keyboard".into(),
			},
			service: SERVICE_NAME.into(),
			version: None,
		},
		scores: output_scores,
	})
}
