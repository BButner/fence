<script lang="ts">
	import type { FenceClientStore } from '$lib/store';
	import { getContext, onDestroy } from 'svelte';
	import { onMount } from 'svelte';
	import { faRefresh } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
	import { listen } from '@tauri-apps/api/event';
	import type { IDisplayScreenshot } from '$lib/types/displays';
	import { invoke } from '@tauri-apps/api';

	const context: FenceClientStore = getContext('fenceClientStore');
	const { displays, regions, screenshots } = context;
	let canvas: HTMLDivElement;
	let factor = 0.1;
	let topOffset = 0;
	let leftOffset = 0;
	let cursorX = 0;
	let cursorY = 0;
	let unlisten: () => void;
	let showScreenshots = false; //TODO This should be stored in the local config

	interface ICursorPositionPayload {
		x: number;
		y: number;
	}

	const drawCanvas = () => {
		topOffset = -$displays.reduce((acc, display) => Math.min(acc, display.top), 0);
		leftOffset = -$displays.reduce((acc, display) => Math.min(acc, display.left), 0);

		const highestRight = $displays.reduce(
			(acc, display) => Math.max(acc, display.left + leftOffset + display.width),
			0
		);
		const highestBottom = $displays.reduce(
			(acc, display) => Math.max(acc, display.top + topOffset + display.height),
			0
		);

		canvas.style.width = `${highestRight * factor}px`;
		canvas.style.height = `${highestBottom * factor}px`;
	};

	const handleShowScreenshots = () => {
		if (showScreenshots) {
			invoke<IDisplayScreenshot[]>('get_display_screenshots').then((result) => {
				console.log(result);
				console.log($displays);
				$screenshots = result;
			});
		} else {
			$screenshots = [];
		}
	};

	onMount(async () => {
		drawCanvas();

		displays.subscribe(() => {
			drawCanvas();
		});

		unlisten = await listen<ICursorPositionPayload>('EVENT_CURSOR_POSITION', (event) => {
			cursorX = event.payload.x;
			cursorY = event.payload.y;
		});
	});

	onDestroy(() => {
		unlisten();
	});
</script>

<div class="p-4 bg-gray-300 dark:bg-gray-950">
	<div>
		<div class="relative" bind:this={canvas}>
			{#each $displays as display}
				<div
					class="bg-gray-400 dark:bg-gray-800 border border-gray-600 dark:border-gray-700"
					style="
					position: absolute;
					top: {display.top * factor + topOffset * factor}px;
					left: {display.left * factor + leftOffset * factor}px;
					width: {display.width * factor}px;
					height: {display.height * factor}px;"
				/>
			{/each}

			{#each $regions as region}
				<div
					class="absolute bg-red-400/10"
					style="
					top: {region.y * factor + topOffset * factor}px;
					left: {region.x * factor + leftOffset * factor}px;
					width: {region.width * factor}px;
					height: {region.height * factor}px;"
				/>
			{/each}

			<div
				class="w-2 h-2 bg-violet-800 rounded-full absolute"
				style="top: {cursorY * factor + topOffset * factor}px; left: {cursorX * factor +
					leftOffset * factor}px"
			/>
		</div>
	</div>

	<div
		class="flex items-center justify-end mt-4 space-x-4 text-xs font-semibold text-gray-700 dark:text-gray-400"
	>
		<div class="flex items-center">
			<input
				bind:checked={showScreenshots}
				on:change={handleShowScreenshots}
				type="checkbox"
				name="showScreenshots"
				id="checkbox-show-screenshots"
			/>
			<label for="checkbox-show-screenshots" class="ml-2"> Show Screenshots </label>
		</div>
		<button
			on:click={async () => await context.refreshDisplays()}
			class="flex items-center space-x-2 py-1 px-5 bg-gray-400/60 dark:bg-gray-800 rounded transition-all hover:bg-gray-400/80 dark:hover:bg-gray-700"
		>
			<Fa icon={faRefresh} />
			<p>Refresh</p>
		</button>
	</div>
</div>
