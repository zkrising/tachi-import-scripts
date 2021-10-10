import { BrowserWindow } from "electron";

let browserWindow: BrowserWindow;

export function SetBrowserWin(win: BrowserWindow) {
	browserWindow = win;
}

export function GetBrowserWin() {
	return browserWindow;
}
