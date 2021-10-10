import { BatchManual, BatchManualScore, integer, Lamps } from "tachi-common";
import logger from "../util/logger";
import SERVICE from "../util/service";
import { ConnectSQLite3 } from "../util/sqlite3";

interface ScoreRow {
	hash: string;
	clear: integer;
	perfect: integer;
	great: integer;
	good: integer;
	bad: integer;
	poor: integer;
	totalnotes: integer;
	maxcombo: integer;
	minbp: integer;
	playcount: integer;
	clearcount: integer;
	failcount: integer;
	rank: integer;
	rate: integer;
	clear_db: integer;
	op_history: integer;
	scorehash: string;
	ghost: string;
	clear_sd: integer;
	clear_ex: integer;
	op_best: integer;
	rseed: integer;
	complete: integer;
}

interface ChartRow {
	hash: string;
	title: string;
	subtitle: string;
	random: 0 | 1 | null;
	mode: 7 | 14;
}

export default function ConvertLR2DB(
	scorePath: string,
	chartPath: string
): BatchManual<"bms:7K" | "bms:14K">[] {
	const scoreDb = ConnectSQLite3(scorePath);
	const chartDb = ConnectSQLite3(chartPath);

	// A non-1 complete means the chart was exited early or something, I think.
	const dbScores: ScoreRow[] = scoreDb
		.prepare(/* sql */ `SELECT * FROM score WHERE complete = 1`)
		.all();

	logger.info(`Found ${dbScores.length} scores.`);

	const scores7K: BatchManualScore<"bms:7K">[] = [];
	const scores14K: BatchManualScore<"bms:14K">[] = [];

	for (const dbScore of dbScores) {
		// I realise this is inefficient. This is a consequence of LR2 using separate databases for
		// songs and charts, instead of just separate tables. Honestly horrific.
		const chart: ChartRow = chartDb
			.prepare(
				/* sql */ `SELECT hash, title, subtitle, random, mode FROM song WHERE hash = ?`
			)
			.get(dbScore.hash);

		if (!chart) {
			logger.warn(
				`Couldn't find chart ${dbScore.hash} in the local song DB. Skipping this score.`
			);
			continue;
		}

		const title = `${chart.title} (${chart.subtitle}) [${dbScore.perfect * 2 + dbScore.great}]`;

		if (chart.random) {
			logger.info(`Skipping score on ${title} as the chart uses #RANDOM.`);
			continue;
		}

		if (![0, 1, 2, 3, 4, 5].includes(dbScore.clear)) {
			logger.warn(`Unknown clear type ${dbScore.clear}. Skipping.`);
			continue;
		}

		const lamp = ConvertLamp(dbScore.clear);

		if (dbScore.minbp === -1 || dbScore.minbp === null) {
			logger.info(
				`Skipping score on ${title} as it had a BP of ${dbScore.minbp}. Probably autoscratch?`
			);
			continue;
		}

		const random = ParseRandom(dbScore.op_best);

		if (!random) {
			continue;
		}

		const score: BatchManualScore<"bms:7K" | "bms:14K"> = {
			identifier: dbScore.hash,
			matchType: "bmsChartHash",
			score: dbScore.perfect * 2 + dbScore.great,
			lamp,
			judgements: {
				pgreat: dbScore.perfect,
				great: dbScore.great,
				good: dbScore.good,
				bad: dbScore.bad,
				poor: dbScore.poor,
			},
			hitMeta: {
				bp: dbScore.minbp,
				maxCombo: dbScore.maxcombo,
			},
			scoreMeta: {
				random,
				client: "LR2",
			},
			timeAchieved: null,
		};

		if (chart.mode === 14) {
			score.scoreMeta!.random = null;
			scores14K.push(score as BatchManualScore<"bms:14K">);
		} else if (chart.mode === 7) {
			scores7K.push(score);
		} else {
			logger.verbose(`Skipping score on unknown playtype ${chart.mode}.`);
		}
	}

	logger.info(`Found ${scores7K.length} 7K scores, and ${scores14K.length} 14K scores.`);

	const batchManuals: BatchManual<"bms:7K" | "bms:14K">[] = [];

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

	return batchManuals;
}

type RanOptions = "NONRAN" | "RANDOM" | "R-RANDOM" | "S-RANDOM" | "MIRROR";

const OP_RAN: RanOptions[] = ["NONRAN", "MIRROR", "RANDOM", "S-RANDOM"];

function ParseRandom(opt: ScoreRow["op_best"]) {
	if (opt > 100) {
		logger.warn(`Unknown play option ${opt}. Skipping.`);
		return null;
	}

	// LR2 stores mods in 21 -> 2, 1 format. This is a bit
	// awkward. What's better? We don't even care about the
	// smaller unit.
	const tenths = (opt - (opt % 10)) / 10;

	const option = OP_RAN[tenths];

	if (!option) {
		logger.warn(`Unknown play option ${tenths}. Skipping.`);
		return null;
	}

	return option;
}

function ConvertLamp(clear: ScoreRow["clear"]): Lamps["bms:7K" | "bms:14K"] {
	if (clear === 0) {
		return "NO PLAY";
	} else if (clear === 1) {
		return "FAILED";
	} else if (clear === 2) {
		return "EASY CLEAR";
	} else if (clear === 3) {
		return "CLEAR";
	} else if (clear === 4) {
		return "HARD CLEAR";
	} else if (clear === 5) {
		return "FULL COMBO";
	}

	throw new Error(`Unknown clear type ${clear}.`);
}
