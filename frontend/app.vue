<script lang="ts">
import { defineComponent } from "vue"
import { IFenceState } from "./lib/types/fence-state"
import { FenceApi } from "./lib/api"
import { IDisplay } from "./lib/types/displays"
import { IRegion } from "./lib/types/regions"

export default defineComponent({
  setup() {
    let state = ref<IFenceState | undefined>(undefined)
    let displays = ref<IDisplay[]>([])
    let regions = ref<IRegion[]>([])

    onMounted(async () => {
      state.value = await FenceApi.getState()
      console.log(state.value)
      displays.value = await FenceApi.getDisplays()
      regions.value = await FenceApi.getRegions()
    })

    provide("state", state)
    provide("displays", displays)
    provide("regions", regions)
  },
})
</script>

<template>
  <Suspense>
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </Suspense>
</template>
