<script lang="ts">
	import type { FenceClientStore } from '$lib/store';
	import { getContext, onDestroy } from 'svelte';
	import { onMount } from 'svelte';
	import clsx from 'clsx';
	import { listen } from '@tauri-apps/api/event';

	const context: FenceClientStore = getContext('fenceClientStore');
	const { displays, regions } = context;
	let canvas: HTMLDivElement;
	let factor = 0.1;
	let topOffset = 0;
	let leftOffset = 0;
	let cursorX = 0;
	let cursorY = 0;
	let unlisten: () => void;

	interface ICursorPositionPayload {
		x: number;
		y: number;
	}

	const drawCanvas = () => {
		topOffset = -$displays.reduce((acc, display) => Math.min(acc, display.top), 0);
		leftOffset = -$displays.reduce((acc, display) => Math.min(acc, display.left), 0);

		console.log('leftOffset', leftOffset);

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

	onMount(async () => {
		drawCanvas();

		displays.subscribe(() => {
			drawCanvas();
			console.log($displays);
		});

		unlisten = await listen<ICursorPositionPayload>('EVENT_CURSOR_POSITION', (event) => {
			// console.log(event.payload.x, event.payload.y);
			cursorX = event.payload.x;
			cursorY = event.payload.y;
		});
	});

	onDestroy(() => {
		unlisten();
	});
</script>

<div class="flex items-start mx-auto my-4">
	<div class="p-4 bg-gray-300 dark:bg-gray-950 rounded shadow-lg">
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
				>
					<img
						class="w-full h-full object-cover"
						src="data:image/png;base64,{display.screenData}"
						alt="display"
					/>
				</div>
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
</div>
