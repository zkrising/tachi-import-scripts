/* eslint-disable no-console */
import { BatchManual, ImportDocument, SuccessfulAPIResponse } from "tachi-common";
import { TISConfig } from "../common/types";
// eslint-disable-next-line @typescript-eslint/no-var-requires
const ipcRenderer = require("electron").ipcRenderer;

type Channels =
	| "LR2_CONVERT"
	| "CONFIG"
	| "IMPORT"
	| "LOG"
	| "UPDATE_API_TOKEN"
	| "BEATORAJA_CONVERT"
	| "USC_CONVERT";

interface Reply {
	LR2_CONVERT: BatchManual[];
	BEATORAJA_CONVERT: BatchManual[];
	USC_CONVERT: BatchManual;
	CONFIG: TISConfig;
	IMPORT: null | SuccessfulAPIResponse<ImportDocument>;
	LOG: boolean;
	UPDATE_API_TOKEN: boolean;
}

interface Content {
	LR2_CONVERT: {
		scorePath: string;
		chartPath: string;
	};
	BEATORAJA_CONVERT: {
		scorePath: string;
		chartPath: string;
	};
	USC_CONVERT: { dbPath: string; playtype: "Controller" | "Keyboard" };
	CONFIG: null;
	IMPORT: BatchManual;
	LOG: {
		level: "info" | "warn" | "error" | "severe" | "crit";
		content: string;
	};
	UPDATE_API_TOKEN: string;
}

export function MakeIPCRequest<C extends Channels>(
	channel: C,
	content: Content[C]
): Promise<Reply[C]> {
	return new Promise((resolve) => {
		console.log(`Making IPC Request ${channel}.`);
		ipcRenderer.on(`$$${channel}`, (e, reply) => {
			console.log(`Got reply ${reply}`);
			resolve(reply as Reply[C]);
		});

		ipcRenderer.send(`@@${channel}`, content);
	});
}
