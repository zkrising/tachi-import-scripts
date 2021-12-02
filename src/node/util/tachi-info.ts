export const BokutachiConfig = {
	name: "Bokutachi",
	baseUrl: "https://bokutachi.xyz",
	clientUrl: "https://bokutachi.xyz",
	clientID: "CI18c4ebe4297a9e66960ad7b7bc88e91ace634ef8",
};

export const BokutachiStagingConfig = {
	name: "Bokutachi Staging",
	baseUrl: "https://staging.bokutachi.xyz",
	clientUrl: "https://staging.bokutachi.xyz",
	clientID: "CI8c4eae44862e6d3b753eef2d2d859ac51d8c516b1",
};

// Get config from env var just incase someone wants to use staging.
export function GetTachiConfig() {
	const TachiConfig = process.env.STAGING ? BokutachiStagingConfig : BokutachiConfig;

	return TachiConfig;
}
