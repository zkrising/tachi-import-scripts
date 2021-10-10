import { CreateLogger } from "mei-logger";
import { format, transports } from "winston";
import SERVICE from "./service";
import Transport from "winston-transport";
import { GetBrowserWin } from "./browser-win";

const projectName = SERVICE;

// Inlined a bunch of the Mei-Logger framework to give
// some leeway.
const baseFormatRoute = format.combine(
	format.timestamp({
		format: "YYYY-MM-DD HH:mm:ss",
	})
);

const formatExcessProperties = (meta: Record<string, unknown>) => {
	let i = 0;
	for (const key in meta) {
		const val = meta[key];

		if (val instanceof Error) {
			meta[key] = { message: val.message, stack: val.stack };
		}
		i++;
	}

	if (!i) {
		return "";
	}

	return ` ${JSON.stringify(meta)}`;
};

const meiPrintf = format.printf(
	({ level, message, context = projectName, timestamp, ...meta }) =>
		`${timestamp} [${
			Array.isArray(context) ? context.join(" | ") : context
		}] ${level}: ${message}${formatExcessProperties(meta)}`
);

const defaultFormatRoute = format.combine(
	baseFormatRoute,
	format.errors({ stack: false }),
	meiPrintf
);

class HTMLLogger extends Transport {
	log(info: any, cb: () => void) {
		const browserWin = GetBrowserWin();

		if (browserWin) {
			browserWin.webContents.send("logger", info);
		}

		cb();
	}
}

const logger = CreateLogger("Tachi Import Scripts", undefined, [
	new transports.File({ filename: "logs/mei.log", format: defaultFormatRoute }),
	new HTMLLogger(),
]);

export default logger;
