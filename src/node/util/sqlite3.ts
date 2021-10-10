import Sqlite3 from "better-sqlite3";
import fs from "fs";
import logger from "./logger";

export function ConnectSQLite3(filepath: string): Sqlite3.Database {
	if (!fs.existsSync(filepath)) {
		logger.error(`File at ${filepath} does not exist.`);
		throw new Error(`File at ${filepath} does not exist.`);
	}

	const db = Sqlite3(filepath);

	return db;
}
