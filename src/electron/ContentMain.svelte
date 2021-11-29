<script lang="ts">
	import { PublicUserDocument } from "tachi-common";

	import { TISConfig } from "../common/types";

	import { GetTachiConfig } from "../node/util/tachi-info";
	const TachiConfig = GetTachiConfig();

	import AuthContainer from "./AuthContainer.svelte";
	import ConvertSelect from "./ConvertSelect.svelte";
	import Divider from "./Divider.svelte";
	import { MakeIPCRequest } from "./ipc-utils";

	export let config: TISConfig;

	let authStatus: "loading" | "no-token" | "invalid-token" | "authed" = "loading";

	let user: PublicUserDocument | null;

	const CheckAuth = () =>
		fetch(TachiConfig.baseUrl + "/api/v1/users/me", {
			headers: {
				Authorization: "Bearer " + config.authToken,
			},
		})
			.then((r) => r.json())
			.then((r) => {
				MakeIPCRequest("UPDATE_API_TOKEN", config!.authToken!);

				if (r.success) {
					authStatus = "authed";
					MakeIPCRequest("LOG", {
						level: "info",
						content: "Successfully authenticated with server!",
					});
					user = r.body;
				} else {
					MakeIPCRequest("LOG", {
						level: "error",
						content: "Failed to authenticate with server, Your token is invalid.",
					});
					authStatus = "invalid-token";
				}
			});

	if (config?.authToken) {
		CheckAuth();
	} else {
		authStatus = "no-token";
	}
</script>

<div class="col-12 text-center">
	<AuthContainer bind:value={config.authToken} CheckAuthAgain={CheckAuth} />
	<hr />
	{#if authStatus === "loading"}
		Loading...
	{:else if authStatus === "no-token"}
		We've got no auth token on record. You'll need to create one.
	{:else if authStatus === "invalid-token"}
		The auth token on record is invalid. Please check your auth key, or re-run the auth key
		creator.
	{:else}
		Authenticated as {user?.username}!
		<Divider />
		<ConvertSelect />
	{/if}
</div>
