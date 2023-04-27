<script>
	
</script>

<div class="relative flex h-full w-full items-center justify-center overflow-hidden">
	<div class="absolute left-0 top-0 z-0 h-96 w-96 -translate-x-1/2 rotate-45 bg-gray-300/60 transition-all hover:bg-red-300/50 dark:bg-gray-800/60 dark:hover:bg-red-600/30"></div>
	<div class="absolute bottom-0 right-0 z-0 h-96 w-96 translate-x-1/2 rotate-45 bg-gray-300/60 transition-all hover:bg-red-300/50 dark:bg-gray-800/50 dark:hover:bg-red-600/30"></div>
	<div class="z-10 space-y-12">
	  <h1 class="bg-gradient-to-br from-violet-500 to-indigo-400 bg-clip-text fill-transparent text-center text-8xl font-semibold tracking-wide text-transparent">
		Fence
	  </h1>
	  <div class="flex items-center space-x-4">
		<input
		  type="text"
		  placeholder="Hostname"
		  class="h-14 w-96 rounded-lg border-4 border-indigo-500 px-2 text-2xl outline-none transition-all focus:ring-8 focus:ring-indigo-500/50 dark:bg-gray-950 dark:text-white"
		/>
		<button
		  class="flex h-14 w-44 items-center justify-between rounded-lg bg-indigo-500/20 px-8 text-xl text-indigo-800 outline-none transition-all hover:bg-indigo-500/30 focus:ring-8 focus:ring-indigo-500/50 dark:text-indigo-300"
		>
		  <!-- {connectionState.status !== ConnectionStatus.CONNECTING && (
			<FontAwesomeIcon icon={faPlug} />
		  )}
		  {connectionState.status === ConnectionStatus.CONNECTING && (
			<span class="animate-spin">
			  <FontAwesomeIcon icon={faCircleNotch} />
			</span>
		  )} -->
		  <p>Connect</p>
		</button>
	  </div>
	</div>
  </div>