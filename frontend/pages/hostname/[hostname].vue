<script lang="ts">
import { defineComponent } from "vue"
const route = useRoute()

export default defineComponent({
  setup() {
    console.log(route)
  },
  data() {
    return {
      hostname: route.params.hostname,
    }
  },
})
</script>

<template>
  <div
    class="w-screen h-screen overflow-hidden grid grid-flow-row grid-rows-[1fr_auto]"
  >
    <div class="flex items-center justify-center">
      <div class="flex space-x-4">
        <DisplayCanvas />
      </div>
    </div>
    <ConnectionStatusBar />
  </div>
</template>
