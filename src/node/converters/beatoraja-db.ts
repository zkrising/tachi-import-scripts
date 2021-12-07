import { BatchManual, BatchManualScore, integer, Lamps } from "tachi-common";
import logger from "../util/logger";
import SERVICE from "../util/service";
import { ConnectSQLite3 } from "../util/sqlite3";

/**
 * @see {@link https://github.com/exch-bms2/beatoraja/blob/3ffb749eb54a8ad760051cdfa01f2d8709abb1be/src/bms/player/beatoraja/ClearType.java}
 */
enum BeatorajaClears {
	NoPlay = 0,
	Failed,
	AssistEasy,
	LightAssistEasy,
	Easy,
	Normal,
	Hard,
	ExHard,
	FullCombo,
	Perfect,
	Max,
}

interface ScoreRow {
	sha256: string;
	mode: integer;
	clear: BeatorajaClears;
	epg: integer;
	egr: integer;
	egd: integer;
	ebd: integer;
	epr: integer;
	ems: integer;
	lpg: integer;
	lgr: integer;
	lgd: integer;
	lbd: integer;
	lpr: integer;
	lms: integer;
	notes: integer;
	combo: integer;
	minbp: integer;
	playcount: integer;
	clearcount: integer;
	trophy: string;
	ghost: string;
	scorehash: string;
	option: integer;
	random: integer;
	date: integer;
	state: integer;
	seed: integer;
}

enum ChartFeatures {
	UNDEFINEDLN = 0b1,
	MINENOTE = 0b10,
	RANDOM = 0b100,
	LONGNOTE = 0b1000,
	CHARGENOTE = 0b10000,
	HELLCHARGENOTE = 0b100000,
	STOPSEQUENCE = 0b1000000,
	SCROLL = 0b10000000,
}

interface ChartRow {
	title: string;
	subtitle: string;
	feature: integer;
	notes: integer;
	mode: 5 | 7 | 14;
}

type RanOptions = "NONRAN" | "RANDOM" | "R-RANDOM" | "S-RANDOM" | "MIRROR";

const randoms: RanOptions[] = ["NONRAN", "MIRROR", "RANDOM", "R-RANDOM", "S-RANDOM"];

export default function ConvertBeatorajaDB(
	scoreFilepath: string,
	chartFilepath: string
): BatchManual<"bms:7K" | "bms:14K">[] {
	// Beatoraja uses separate sqlite3 dbs instead of multiple tables inside the same database.
	// What??
	const scoreDB = ConnectSQLite3(scoreFilepath);
	const chartDB = ConnectSQLite3(chartFilepath);

	const dbScores: ScoreRow[] = scoreDB
		.prepare(/* sql */ `SELECT * FROM score WHERE mode = 0`)
		.all();

	logger.info(
		`Found ${dbScores.length} scores. Note that this processor will take a while, as it has to crossreference data.`
	);

	const scores7K: BatchManual<"bms:7K">["scores"] = [];
	const scores14K: BatchManual<"bms:14K">["scores"] = [];

	// Yes. I realise having a nested sql query is an antipattern, however, I can't join
	// across databases.
	for (const dbScore of dbScores) {
		const chart: ChartRow = chartDB
			.prepare(
				/* sql */ `SELECT title, subtitle, feature, notes, mode FROM song WHERE sha256 = ?`
			)
			.get(dbScore.sha256);

		if (!chart) {
			logger.warn(
				`Couldn't find a matching chart for score ${dbScore.sha256}. Unable to verify integrity.`
			);
			continue;
		}

		const name = `${chart.title} ${chart.subtitle}`;

		if ((chart.feature & ChartFeatures.RANDOM) === ChartFeatures.RANDOM) {
			logger.info(`Skipping ${name} as it has #RANDOM declarations.`);
			continue;
		}

		if (dbScore.notes !== chart.notes) {
			logger.warn(
				`${name} - Score notecount ${dbScore.notes} does not match chart notecount ${chart.notes}? Skipping.`
			);
			continue;
		}

		if (chart.mode === 5) {
			logger.debug(`Skipping 5K score on ${name}.`);
			continue;
		}

		let random: RanOptions | null = null;

		if (chart.mode === 7) {
			random = randoms[dbScore.random];

			if (!random) {
				logger.info(
					`Skipping score on ${name} as the random option was invalid or unfair (H-Ran, Spiral, etc.).`
				);
				continue;
			}
		}

		let bp: number | null = dbScore.minbp;

		if (bp === 2 ** 31 - 1) {
			bp = null;
		} else if (bp === -1) {
			bp = null;
		}

		const score: BatchManualScore<"bms:7K" | "bms:14K"> = {
			identifier: dbScore.sha256,
			matchType: "bmsChartHash",
			score: (dbScore.lpg + dbScore.epg) * 2 + dbScore.egr + dbScore.lgr,
			hitMeta: {
				// beatoraja uses 2 ** 31 - 1 instead of null.
				bp,
				fast: dbScore.egr + dbScore.egd,
				slow: dbScore.lgr + dbScore.lgd,
				maxCombo: dbScore.combo,
				// gauge?
				epg: dbScore.epg,
				egr: dbScore.egr,
				egd: dbScore.egd,
				ebd: dbScore.ebd,
				epr: dbScore.epr,
				lpg: dbScore.lpg,
				lgr: dbScore.lgr,
				lgd: dbScore.lgd,
				lbd: dbScore.lbd,
				lpr: dbScore.lpr,
			},
			judgements: {
				pgreat: dbScore.epg + dbScore.lpg,
				great: dbScore.egr + dbScore.lgr,
				good: dbScore.egd + dbScore.lgd,
				bad: dbScore.ebd + dbScore.lbd,
				// ???
				poor: dbScore.epr + dbScore.lpr + dbScore.ems + dbScore.lms,
			},
			timeAchieved: dbScore.date * 1000,
			scoreMeta: {
				client: "lr2oraja",
				inputDevice: null,
				random,
			},
			lamp: ConvertLamp(dbScore.clear),
		};

		if (chart.mode === 7) {
			scores7K.push(score);
		} else if (chart.mode === 14) {
			// just triple-enforce that this is definitely nulled,
			// as beatoraja only stores the left-hand random for DP random.
			// because it's great like that.
			score.scoreMeta!.random = null;
			scores14K.push(score as BatchManualScore<"bms:14K">);
		}
	}

	logger.info(`Found ${scores7K.length} 7K Scores and ${scores14K.length} 14K scores.`);

	const batchManuals: BatchManual<"bms:7K" | "bms:14K">[] = [];

	if (scores14K.length !== 0) {
		batchManuals.push({
			meta: {
				game: "bms",
				playtype: "14K",
				service: SERVICE,
			},
			scores: scores14K,
		});
	}

	if (scores7K.length !== 0) {
		batchManuals.push({
			meta: {
				game: "bms",
				playtype: "7K",
				service: SERVICE,
			},
			scores: scores7K,
		});
	}

	return batchManuals;
}

function ConvertLamp(clear: BeatorajaClears): Lamps["bms:7K" | "bms:14K"] {
	switch (clear) {
		case BeatorajaClears.NoPlay:
			return "NO PLAY";
		case BeatorajaClears.Failed:
			return "FAILED";
		case BeatorajaClears.AssistEasy:
			return "ASSIST CLEAR";
		case BeatorajaClears.LightAssistEasy:
			return "ASSIST CLEAR";
		case BeatorajaClears.Easy:
			return "EASY CLEAR";
		case BeatorajaClears.Normal:
			return "CLEAR";
		case BeatorajaClears.Hard:
			return "HARD CLEAR";
		case BeatorajaClears.ExHard:
			return "EX HARD CLEAR";
		case BeatorajaClears.FullCombo:
		case BeatorajaClears.Perfect:
		case BeatorajaClears.Max:
			return "FULL COMBO";
	}
}
