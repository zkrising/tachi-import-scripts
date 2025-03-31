/* eslint-disable no-console */
import { type BatchManual,type  ImportDocument,type  SuccessfulAPIResponse } from "tachi-common";
import { type TISConfig } from "./common/types";
import { invoke } from "@tauri-apps/api/core";

type Channels =
	| "lr2_convert"
	| "config"
	| "import"
	| "log"
	| "update_api_token"
	| "beatoraja_convert"
	| "usc_convert";

interface Reply {
	lr2_convert: {
		k7: BatchManual | null;
		k14: BatchManual | null;
	};
	beatoraja_convert: {
		k7: BatchManual | null;
		k14: BatchManual | null;
	};
	usc_convert: BatchManual;
	config: TISConfig;
	import: null | SuccessfulAPIResponse<ImportDocument>;
	log: boolean;
	update_api_token: boolean;
}

interface Content {
	lr2_convert: {opts: {
		scorePath: string;
		chartPath: string;
	}};
	beatoraja_convert: {opts: {
		scorePath: string;
		chartPath: string;
	}};
	usc_convert: {opts: { dbPath: string; playtype: "Controller" | "Keyboard" } };
	config: {};
	import: { bm: BatchManual };
	log: {
		level: "info" | "warn" | "error";
		content: string;
	};
	update_api_token: {token: string};
}

export function MakeIPCRequest<C extends Channels>(
	channel: C,
	content: Content[C]
): Promise<Reply[C]> {
	return invoke(channel, content).catch(err => {
		if (channel !== "log" ) {
			MakeIPCRequest("log", {level: "error", content: err})
		}
	}) as any;
}
