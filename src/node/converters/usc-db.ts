import { BatchManual, integer, Lamps } from "tachi-common";
import logger from "../util/logger";
import SERVICE from "../util/service";
import { ConnectSQLite3 } from "../util/sqlite3";

enum GaugeType {
	NORMAL = 0,
	HARD = 1,
}

interface ScoreRow {
	score: number;
	crit: number;
	near: number;
	miss: number;

	/**
	 * Gauge is between 0 and 1, not 0 and 100!
	 */
	gauge: number;

	/**
	 * Legacy mod info, no longer used.
	 */
	gameflags: number;

	/**
	 * Time the score was achieved in Unix Seconds.
	 */
	timestamp: number;

	/**
	 * Path to the replay file.
	 */
	replay: string;
	chart_hash: string;
	user_name: string;
	user_id: string;

	/**
	 * No idea what this is, it seems to generally be set as 1.
	 */
	local_score: number;

	/**
	 * Hit windows for the game, There are only three we accept.
	 */
	window_perfect: number;
	window_good: number;
	window_hold: number;
	window_miss: number;
	window_slam: number;

	/**
	 * Whether this was performed on NORMAL (0) or HARD (1).
	 */
	gauge_type: GaugeType;
	/**
	 * Bitwise flags for the auto_things. I don't know what each value
	 * corresponds to, but I also don't care, since we reject any non-zero.
	 */
	auto_flags: number;
	/**
	 * Looks to be unused, but would be used for gauge-specific info, like
	 * the difficulty level of blaster gauge. Probably. We don't care.
	 */
	gauge_opt: number;
	/**
	 * Whether mirror is being used or not.
	 */
	mirror: 0 | 1;
	/**
	 * Whether random is being used or not.
	 */
	random: 0 | 1;

	/**
	 * Early, Late and maxCombo info
	 */
	early: integer | null;
	late: integer | null;
	combo: integer | null;

	/**
	 * We attach these two properties on for logging.
	 */
	title: string;
	diff_shortname: string;
}

function GetNoteMod(dbScore: ScoreRow): "NORMAL" | "MIRROR" | "RANDOM" | "MIR-RAN" {
	if (dbScore.mirror && dbScore.random) {
		return "MIR-RAN";
	} else if (dbScore.mirror) {
		return "MIRROR";
	} else if (dbScore.random) {
		return "RANDOM";
	}
	return "NORMAL";
}

function GetLamp(dbScore: ScoreRow): Lamps["usc:Controller" | "usc:Keyboard"] {
	if (dbScore.score === 10_000_000) {
		return "PERFECT ULTIMATE CHAIN";
	} else if (dbScore.miss === 0) {
		return "ULTIMATE CHAIN";
	} else if (dbScore.gauge_type === GaugeType.HARD) {
		if (dbScore.gauge > 0) {
			return "EXCESSIVE CLEAR";
		}

		return "FAILED";
	}

	if (dbScore.gauge >= 0.7) {
		return "CLEAR";
	}
	return "FAILED";
}

interface HitWindows {
	crit: number;
	near: number;
	slam: number;
	miss: number;
	hold: number;
}

/**
 * Default hit windows as of the hit window update.
 */
const DEFAULT_HIT_WINDOWS: HitWindows = {
	crit: 46,
	near: 150,
	hold: 150,
	miss: 300,
	slam: 84,
};

/**
 * Old hit windows. These are universally tighter
 * than the new update, so we're going to allow them.
 */
const LEGACY_HIT_WINDOWS: HitWindows = {
	crit: 46,
	near: 92,
	hold: 138,
	miss: 250,
	slam: 84,
};

/**
 * A bug occurs if users updated an old version of the game
 * for the hit window update, where only the miss window would
 * change.
 * We're going to allow this aswell.
 */
const BUGGED_HIT_WINDOWS: HitWindows = {
	crit: 46,
	near: 92,
	hold: 138,
	miss: 300,
	slam: 84,
};

function AreHitWindowsEqual(dbScore: ScoreRow, hitWindows: HitWindows) {
	return (
		dbScore.window_good !== hitWindows.near ||
		dbScore.window_hold !== hitWindows.hold ||
		dbScore.window_miss !== hitWindows.miss ||
		dbScore.window_perfect !== hitWindows.crit ||
		dbScore.window_slam !== hitWindows.slam
	);
}

function FormatDbScore(dbScore: ScoreRow) {
	return `${dbScore.title} [${dbScore.diff_shortname}] (${dbScore.score.toLocaleString()})`;
}

function ValidateHitWindows(dbScore: ScoreRow) {
	if (AreHitWindowsEqual(dbScore, DEFAULT_HIT_WINDOWS)) {
		return true;
	}

	if (AreHitWindowsEqual(dbScore, LEGACY_HIT_WINDOWS)) {
		logger.verbose(`Allowing score with legacy hit windows to be imported anyway.`);
		return true;
	}

	if (AreHitWindowsEqual(dbScore, BUGGED_HIT_WINDOWS)) {
		logger.warn(
			`Score (${FormatDbScore(
				dbScore
			)}) detected with bugged hit windows! A game update has caused the new hitwindows to partially apply. YOU SHOULD GO INTO SETTINGS AND RESET YOUR HIT WINDOWS, AS YOU ARE PLAYING ON TIGHTER HIT WINDOWS THAN NORMAL!`
		);
		logger.warn(`For compatibility reasons, this score will be accepted.`);
		return true;
	}

	return false;
}

export default function ConvertUSCDB(
	filepath: string,
	playtype: "Controller" | "Keyboard"
): BatchManual<"usc:Controller" | "usc:Keyboard"> {
	const db = ConnectSQLite3(filepath);

	const version: number = db.prepare(/* sql */ `SELECT version FROM Database`).get().version;

	if (version < 19) {
		logger.error(
			`The version of your maps.db is ${version}, which is below the minimum of 19. Update your game. Refusing to run.`
		);

		throw new Error(
			`The version of your maps.db is ${version}, which is below the minimum of 19. Update your game. Refusing to run.`
		);
	}

	if (version > 20) {
		logger.error(
			`The version of your maps.db is ${version}, which is a version after what this tool supports (20). It might not be safe to convert this. Report this, and I'll update the tool to work for the later version!`
		);

		throw new Error(
			`The version of your maps.db is ${version}, which is a version after what this tool supports (20). It might not be safe to convert this. Report this, and I'll update the tool to work for the later version!`
		);
	}

	const dbScores: ScoreRow[] = db
		.prepare(
			/* sql */ `SELECT * FROM Scores LEFT JOIN Charts ON Scores.chart_hash = Charts.hash`
		)
		.all();

	logger.info(`Found ${dbScores.length} scores.`);

	const scores: BatchManual<"usc:Controller" | "usc:Keyboard">["scores"] = [];

	for (const dbScore of dbScores) {
		if (!ValidateHitWindows(dbScore)) {
			logger.error(
				`Invalid Hit Windows for score ${FormatDbScore(dbScore)}. Skipping.`,
				dbScore
			);
			continue;
		}

		scores.push({
			score: dbScore.score,
			identifier: dbScore.chart_hash,
			matchType: "uscChartHash",
			hitMeta: {
				gauge: dbScore.gauge,
				fast: dbScore.early,
				slow: dbScore.late,
				maxCombo: dbScore.combo,
			},
			timeAchieved: dbScore.timestamp * 1000,
			judgements: {
				critical: dbScore.crit,
				near: dbScore.near,
				miss: dbScore.miss,
			},
			scoreMeta: {
				gaugeMod: dbScore.gauge_type === GaugeType.NORMAL ? "NORMAL" : "HARD",
				noteMod: GetNoteMod(dbScore),
			},
			lamp: GetLamp(dbScore),
		});

		logger.debug(`Successfully converted ${FormatDbScore(dbScore)}.`);
	}

	logger.info(`Converted into ${scores.length} scores.`);

	return {
		meta: {
			game: "usc",
			playtype,
			service: SERVICE,
		},
		scores,
	};
}
