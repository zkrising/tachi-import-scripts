import { writable, Writable } from "svelte/store";
import { TISConfig } from "../common/types";

export const config: Writable<TISConfig> = writable({
	authToken: null,
	warning: "stub",
});
