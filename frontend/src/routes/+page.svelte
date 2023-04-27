<script lang="ts">
	import { getContext } from 'svelte';
	import { onMount } from 'svelte';
	import type { FenceClientStore } from '../lib/store';
	import NewConnection from '../components/connection/NewConnection.svelte';
	import PreviousConnections from '../components/connection/PreviousConnections.svelte';

	let hostname = '';
	const context: FenceClientStore = getContext('fenceClientStore');
	const { grpcStatus } = context;

	onMount(() => {
		console.log($grpcStatus);
	});
</script>

<div class="grid grid-flow-col grid-cols-[320px_1fr]">
	<PreviousConnections />
	<NewConnection />
</div>
