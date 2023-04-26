<script lang="ts">
	import { getContext } from "svelte";
	import { onMount } from "svelte";
	import type { FenceClientStore } from "../lib/store";

	let hostname = '';
	const context: FenceClientStore = getContext("fenceClientStore");
	const { grpcStatus } = context;

	onMount(() => {
		console.log($grpcStatus)
	})
</script>