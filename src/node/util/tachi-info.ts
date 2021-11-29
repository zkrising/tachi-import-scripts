export const BokutachiConfig = {
	name: "Bokutachi",
	baseUrl: "https://bokutachi.xyz",
	clientUrl: "https://bokutachi.xyz",
	clientID: "todo",
};

export const BokutachiStagingConfig = {
	name: "Bokutachi Staging",
	baseUrl: "https://staging.bokutachi.xyz",
	clientUrl: "https://staging.bokutachi.xyz",
	clientID: "CI8c4eae44862e6d3b753eef2d2d859ac51d8c516b1",
};

export function GetTachiConfig() {
	const TachiConfig = process.env.STAGING ? BokutachiStagingConfig : BokutachiConfig;

	return TachiConfig;
}
