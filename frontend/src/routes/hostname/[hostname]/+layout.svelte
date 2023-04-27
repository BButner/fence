<script>
	import ConnectionStatusBar from '../../../components/connection/ConnectionStatusBar.svelte';
</script>

<div class="grid h-screen grid-flow-row grid-rows-[1fr_auto]">
	<slot />
	<ConnectionStatusBar />
</div>
