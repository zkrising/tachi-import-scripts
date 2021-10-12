/* eslint-disable no-await-in-loop */
import { app, BrowserWindow, ipcMain, shell } from "electron";
import path from "path";
import ConvertLR2DB from "./node/converters/lr2-db";
import { SetBrowserWin } from "./node/util/browser-win";
import { TIS_CONFIG, UpdateConfig } from "./node/util/config";
import logger from "./node/util/logger";
import TachiConfig from "./node/util/tachi-info";
import fetch from "node-fetch";
import {
	BatchManual,
	FormatGame,
	ImportDocument,
	SuccessfulAPIResponse,
	UnsuccessfulAPIResponse,
} from "tachi-common";
import fs from "fs";
import ConvertUSCDB from "./node/converters/usc-db";
import ConvertBeatorajaDB from "./node/converters/beatoraja-db";

export let win: BrowserWindow;

app.on("ready", () => {
	win = new BrowserWindow({
		title: "Tachi Import Scripts",
		webPreferences: {
			preload: path.join(__dirname, "./preload.js"),
			nodeIntegration: true,
			contextIsolation: false,
			nativeWindowOpen: false,
		},
	});

	SetBrowserWin(win);

	win.webContents.setWindowOpenHandler(({ url }) => {
		logger.info(`Navigating to ${url}.`);
		shell.openExternal(url);

		return { action: "deny" };
	});

	win.loadFile(path.join(__dirname, "../pages/index.html"));

	win.on("ready-to-show", () => {
		logger.info(`Welcome to ${TachiConfig.name} Import Scripts!`);
		logger.info(
			`This console will log various script information. You'll know if something goes wrong!`
		);
	});

	ipcMain.on("@@LR2_CONVERT", (e, { scorePath, chartPath }) => {
		const res = ConvertLR2DB(scorePath, chartPath);

		logger.info(`Conversion Complete.`);

		UpdateConfig({
			lr2DB: {
				scorePath,
				chartPath,
			},
		});

		e.reply("$$LR2_CONVERT", res);
	});

	ipcMain.on("@@BEATORAJA_CONVERT", (e, { scorePath, chartPath }) => {
		const res = ConvertBeatorajaDB(scorePath, chartPath);

		logger.info(`Conversion Complete.`);

		UpdateConfig({
			beatorajaDB: {
				scorePath,
				chartPath,
			},
		});

		e.reply("$$BEATORAJA_CONVERT", res);
	});

	ipcMain.on("@@USC_CONVERT", (e, dbPath) => {
		const res = ConvertUSCDB(dbPath);

		logger.info(`Conversion Complete.`);

		UpdateConfig({
			uscDB: {
				dbPath,
			},
		});

		e.reply("$$USC_CONVERT", res);
	});

	ipcMain.on("@@LOG", (e, { level, content }) => {
		// @ts-expect-error lazy
		logger[level](content);

		e.reply("$$LOG", true);
	});

	ipcMain.on("@@UPDATE_API_TOKEN", (e, authToken) => {
		UpdateConfig({
			authToken,
		});

		e.reply("$$UPDATE_API_TOKEN", true);
	});

	ipcMain.on("@@CONFIG", (e) => {
		const config = TIS_CONFIG;

		logger.info(`Retrieved config.`);

		e.reply("$$CONFIG", config);
	});

	ipcMain.on("@@IMPORT", async (e, batchManual: BatchManual) => {
		if (!TIS_CONFIG.authToken) {
			logger.error(`Can't send an import without an authToken.`);
			return null;
		}

		logger.info(
			`Making import request to ${TachiConfig.baseUrl} for ${FormatGame(
				batchManual.meta.game,
				batchManual.meta.playtype
			)}.`
		);

		try {
			const res = await fetch(`${TachiConfig.baseUrl}/ir/direct-manual/import`, {
				headers: {
					Authorization: `Bearer ${TIS_CONFIG.authToken}`,
					"Content-Type": "application/json",
					"X-User-Intent": "true",
				},
				method: "POST",
				body: JSON.stringify(batchManual),
			});

			if (!res) {
				return e.reply("$$IMPORT", null);
			}

			const rj = (await res.json()) as
				| SuccessfulAPIResponse<ImportDocument>
				| UnsuccessfulAPIResponse;

			if (rj.success) {
				logger.info(
					`Successfully imported scores for ${FormatGame(
						batchManual.meta.game,
						batchManual.meta.playtype
					)}.`
				);

				logger.info(
					`New Scores: ${rj.body.scoreIDs.length} | Failed: ${rj.body.errors.length}.`
				);

				return e.reply("$$IMPORT", rj);
			} else if (res.status >= 500) {
				logger.error(
					`Server error in importing scores: ${rj.description} (${res.status} ${res.statusText}).`
				);
			} else {
				logger.error(
					`Error in importing scores: ${rj.description} (${res.status} ${res.statusText})`
				);
			}

			const filename = `${Date.now()}-${batchManual.meta.game}-${batchManual.meta.playtype}`;
			logger.info(`Saving this import document to batch-manual/${filename} for debugging.`);

			// mkdir -p batch-manual
			fs.mkdirSync("batch-manual", { recursive: true });
			fs.writeFileSync(
				`batch-manual/${filename}`,
				JSON.stringify(batchManual, null, "\t"),
				{}
			);

			return e.reply("$$IMPORT", null);
		} catch (err) {
			logger.error(`Request to ${TachiConfig.baseUrl} failed. ${(err as Error).message}`);
			return e.reply("$$IMPORT", null);
		}
	});
});
