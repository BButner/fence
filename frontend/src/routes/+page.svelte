<script lang="ts">
	import type { FenceClientStore } from "$lib/store";
	import { getContext } from "svelte";
	import { onMount } from "svelte";

	let hostname = '';
	const context: FenceClientStore = getContext("fenceClientStore");
	const { grpcStatus } = context;

	onMount(() => {
		console.log($grpcStatus)
	})
</script>

<input type="text" bind:value={hostname}>

<button on:click={() => context.connectGrpc(hostname)}>Connect</button>
<p>{$grpcStatus}</p>