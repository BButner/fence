<script lang="ts">
	import Fa from 'svelte-fa';
	import { faCircleNotch, faPlug } from '@fortawesome/free-solid-svg-icons';
	import { clsx } from 'clsx';
	import type { FenceClientStore } from '$lib/store';
	import { GrpcStatus, type IGrpcEventPayload } from '$lib/types/grpc-status';
	import { getContext } from 'svelte';
	import { onDestroy, onMount } from 'svelte';
	import { listen } from '@tauri-apps/api/event';

	let context: FenceClientStore = getContext('fenceClientStore');
	let connectingHostname = '';
	let connectionInvalid = false;
	let unlisten: () => void;

	onMount(async () => {
		unlisten = await listen<IGrpcEventPayload>('EVENT_GRPC_STATUS', (event) => {
			let newStatus = GrpcStatus[event.payload.event as keyof typeof GrpcStatus];

			if (newStatus === GrpcStatus.CONNECTED) {
				let encodedUri = encodeURIComponent(connectingHostname);
				window.location.href = `/hostname/${encodedUri}`;
			}
		});
	});

	const attemptConnect = async () => {
		if (!connectingHostname) {
			connectionInvalid = true;
			setInvalidTimeout();
			return;
		}

		try {
			new URL(connectingHostname);
		} catch (e) {
			connectionInvalid = true;
			setInvalidTimeout();
			return;
		}

		context.connectGrpc(connectingHostname);
	};

	const setInvalidTimeout = () => {
		setTimeout(() => {
			connectionInvalid = false;
		}, 2000);
	};

	onDestroy(() => {
		unlisten();
	});

	const { grpcStatus } = context;
</script>

<div class="relative flex h-full w-full items-center justify-center overflow-hidden">
	<div
		class="absolute left-0 top-0 z-0 h-96 w-96 -translate-x-1/2 rotate-45 bg-gray-300/60 transition-all hover:bg-red-300/50 dark:bg-gray-800/60 dark:hover:bg-red-600/30"
	/>
	<div
		class="absolute bottom-0 right-0 z-0 h-96 w-96 translate-x-1/2 rotate-45 bg-gray-300/60 transition-all hover:bg-red-300/50 dark:bg-gray-800/50 dark:hover:bg-red-600/30"
	/>
	<div class="z-10 space-y-12">
		<h1
			class="bg-gradient-to-br from-violet-500 to-indigo-400 bg-clip-text fill-transparent text-center text-8xl font-semibold tracking-wide text-transparent"
		>
			Fence
		</h1>
		<div class="flex items-center space-x-4">
			<input
				bind:value={connectingHostname}
				type="text"
				placeholder="Hostname"
				class={clsx(
					'h-14 w-96 rounded-lg border-4 border-indigo-500 px-2 text-2xl outline-none transition-all focus:ring-8 focus:ring-indigo-500/50 dark:bg-gray-950 dark:text-white',
					connectionInvalid &&
						'border-red-500/50 focus:ring-red-500/50 dark:border-red-500/50 dark:focus:ring-red-500/50'
				)}
			/>
			<button
				on:click={attemptConnect}
				class={clsx(
					'flex h-14 w-44 items-center justify-between rounded-lg bg-indigo-500/20 px-8 text-xl text-indigo-800 outline-none transition-all hover:bg-indigo-500/30 focus:ring-8 focus:ring-indigo-500/50 dark:text-indigo-300',
					connectionInvalid &&
						'bg-red-500/20 hover:bg-red-500/40 text-red-800 dark:text-red-300 focus:ring-red-500/50'
				)}
			>
				{#if $grpcStatus !== GrpcStatus.CONNECTING}
					<Fa icon={faPlug} />
				{/if}
				{#if $grpcStatus === GrpcStatus.CONNECTING}
					<span class="animate-spin">
						<Fa icon={faCircleNotch} />
					</span>
				{/if}
				<p>Connect</p>
			</button>
		</div>
	</div>
</div>
