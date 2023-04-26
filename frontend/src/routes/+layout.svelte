<script lang="ts">
	import "../app.scss"
	export const fenceClientStore = new FenceClientStore();

	import { onMount } from "svelte"
	import { FenceClientStore } from "../lib/store";
	import { setContext } from "svelte";

	setContext("fenceClientStore", fenceClientStore)

	onMount(() => {
		// Update the state every time we refresh
		fenceClientStore.updateFromState();
	})
</script>

<slot/>