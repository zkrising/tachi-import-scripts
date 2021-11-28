import fs from "fs";
import logger from "./logger";
import p from "prudence";
import { FormatPrError } from "./prudence";
import { TISConfig } from "../../common/types";
import deepmerge from "deepmerge";

function GetConfig() {
	try {
		// Write a new config file if one doesn't exist.
		if (!fs.existsSync("tis-config.json")) {
			fs.writeFileSync(
				"tis-config.json",
				JSON.stringify(
					{
						warning: "THIS FILE WILL CONTAIN AN API AUTH KEY. DON'T SEND IT TO ANYONE!",
					},
					null,
					"\t"
				)
			);
		}

		const data = fs.readFileSync("tis-config.json", "utf-8");
		const json = JSON.parse(data);

		const err = p(
			json,
			{
				lr2DB: p.optional({
					scoreDB: "*string",
					chartPath: "*string",
				}),
				beatorajaDB: p.optional({
					scoreDB: "*string",
					chartPath: "*string",
				}),
				uscDB: p.optional({
					dbPath: "*string",
					playtype: p.optional(p.isIn("Controller", "Keyboard")),
				}),
				authToken: "?string",
				warning: p.any,
			},
			{
				uscDB: {
					playtype: "Expected Controller, Keyboard, or no property.",
				},
			}
		);

		if (err) {
			logger.error(`Error in tis-config.json. ${FormatPrError(err)}.`, { json });
		}

		return json as TISConfig;
	} catch (err) {
		logger.error(err);
		throw err;
	}
}

export function UpdateConfig(partialConfig: Partial<TISConfig>) {
	const newConfig = deepmerge(TIS_CONFIG, partialConfig);

	fs.writeFileSync("tis-config.json", JSON.stringify(newConfig, null, "\t"));

	TIS_CONFIG = newConfig;

	logger.info(`Updated Config File.`);
}

export let TIS_CONFIG = GetConfig();
