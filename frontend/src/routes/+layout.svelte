<script lang="ts">
	import '../app.scss';
	export const fenceClientStore = new FenceClientStore();

	import { onDestroy, onMount } from 'svelte';
	import { FenceClientStore } from '../lib/store';
	import { setContext } from 'svelte';
	import { emit, listen } from '@tauri-apps/api/event';
	import { GrpcStatus, type IGrpcEventPayload } from '$lib/types/grpc-status';

	setContext('fenceClientStore', fenceClientStore);

	let unlisten: () => void;

	onMount(async () => {
		// Update the state every time we refresh
		fenceClientStore.updateFromState();

		unlisten = await listen<IGrpcEventPayload>('EVENT_GRPC_STATUS', (event) => {
			fenceClientStore.grpcStatus.set(GrpcStatus[event.payload.event as keyof typeof GrpcStatus]);
		});
	});

	onDestroy(() => {
		unlisten();
	});
</script>

<slot />
