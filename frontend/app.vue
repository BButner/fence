<script lang="ts">
import { defineComponent } from "vue"
import { IFenceState } from "./lib/types/fence-state"
import { FenceApi } from "./lib/api"
import { IDisplay } from "./lib/types/displays"
import { IRegion } from "./lib/types/regions"
import { library } from "@fortawesome/fontawesome-svg-core"
import { faPlug } from "@fortawesome/free-solid-svg-icons"

export default defineComponent({
  setup() {
    let state = ref<IFenceState | undefined>(undefined)
    let displays = ref<IDisplay[]>([])
    let regions = ref<IRegion[]>([])
    const router = useRouter()

    onMounted(async () => {
      state.value = await FenceApi.getState()
      displays.value = await FenceApi.getDisplays()
      regions.value = await FenceApi.getRegions()

      router.afterEach(async (to, from) => {
        state.value = await FenceApi.getState()
        displays.value = await FenceApi.getDisplays()
        regions.value = await FenceApi.getRegions()
      })
    })

    provide("state", state)
    provide("displays", displays)
    provide("regions", regions)

    library.add(faPlug)

    return {
      state,
    }
  },
})
</script>

<template>
  <EventHandler />
  <Suspense>
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </Suspense>
</template>
