<script lang="ts">
	import { faCircleNotch, faPlug, faPlugCircleXmark } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
	import clsx from 'clsx';
	import type { FenceClientStore } from '$lib/store';
	import { getContext } from 'svelte';
	import { GrpcStatus } from '$lib/types/grpc-status';

	const context: FenceClientStore = getContext('fenceClientStore');
	const { hostname, grpcStatus } = context;

	const statusButtonClick = () => {
		if (
			[GrpcStatus.CONNECTION_LOST, GrpcStatus.DISCONNECTED, GrpcStatus.ERROR].includes(
				$grpcStatus
			) &&
			$hostname
		) {
			context.connectGrpc($hostname);
		}
	};
</script>

<div class="flex h-6 w-full items-center bg-gray-300 text-sm dark:bg-gray-800">
	<button
		on:click={statusButtonClick}
		class={clsx(
			'flex h-6 items-center space-x-2 px-4 transition-all',
			$grpcStatus === GrpcStatus.CONNECTED &&
				'bg-green-400/50 text-green-900 dark:bg-green-400/20 dark:text-green-400',
			$grpcStatus === GrpcStatus.CONNECTING &&
				'bg-yellow-400/50 text-yellow-900 dark:bg-yellow-400/20 dark:text-yellow-400',
			($grpcStatus === GrpcStatus.DISCONNECTED ||
				$grpcStatus === GrpcStatus.CONNECTION_LOST ||
				$grpcStatus === GrpcStatus.ERROR) &&
				'bg-red-400/50 text-red-900 dark:bg-red-400/20 dark:text-red-400'
		)}
	>
		<div class="flex w-6 items-center">
			{#if $grpcStatus === GrpcStatus.CONNECTED}
				<Fa icon={faPlug} />
			{/if}
			{#if $grpcStatus === GrpcStatus.CONNECTING}
				<Fa icon={faCircleNotch} class="m-0 h-4 origin-center animate-spin p-0" />
			{/if}
			{#if [GrpcStatus.CONNECTION_LOST, GrpcStatus.DISCONNECTED, GrpcStatus.ERROR].includes($grpcStatus)}
				<Fa icon={faPlugCircleXmark} class="animate-pulse" />
			{/if}
		</div>
		<span>{$hostname}</span>
	</button>
</div>
