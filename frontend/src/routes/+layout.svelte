<script lang="ts">
	import '../app.scss';
	export const fenceClientStore = new FenceClientStore();

	import { onDestroy, onMount } from 'svelte';
	import { FenceClientStore } from '../lib/store';
	import { setContext } from 'svelte';
	import { emit, listen } from '@tauri-apps/api/event';
	import type { IGrpcEventPayload } from '$lib/types/grpc-status';

	setContext('fenceClientStore', fenceClientStore);

	let unlisten: () => void;

	onMount(async () => {
		// Update the state every time we refresh
		fenceClientStore.updateFromState();

		unlisten = await listen<IGrpcEventPayload>('EVENT_GRPC_STATUS', (event) => {
			console.log('Setting status to ', event.payload.event);
			fenceClientStore.grpcStatus.set(event.payload.event);
		});
	});

	onDestroy(() => {
		unlisten();
	});
</script>

<slot />
