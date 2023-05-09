<script lang="ts">
import { listen, UnlistenFn } from "@tauri-apps/api/event"
import { defineComponent } from "vue"
import { ICursorPosition } from "~/lib/types/cursor"
import { IDisplay } from "~/lib/types/displays"
import { CURSOR_POSITION } from "~/lib/types/events"
import { IRegion } from "~/lib/types/regions"

export default defineComponent({
  setup() {
    const state = inject("state")
    const displays = inject<Ref<IDisplay[]>>("displays")
    const regions = inject<Ref<IRegion[]>>("regions")
    const canvasWidth = ref(0)
    const canvasHeight = ref(0)
    const topOffset = ref(0)
    const leftOffset = ref(0)
    const factor = ref(0.1)
    const cursorX = ref(0)
    const cursorY = ref(0)
    const listener = ref<UnlistenFn | undefined>(undefined)

    const updateCanvas = () => {
      if (displays) {
        topOffset.value = -displays.value.reduce(
          (acc, display) => Math.min(acc, display.top),
          0,
        )
        leftOffset.value = -displays.value.reduce(
          (acc, display) => Math.min(acc, display.left),
          0,
        )

        const highestRight = displays.value.reduce(
          (acc, display) =>
            Math.max(acc, display.left + leftOffset.value + display.width),
          0,
        )
        const highestBottom = displays.value.reduce(
          (acc, display) =>
            Math.max(acc, display.top + topOffset.value + display.height),
          0,
        )

        canvasWidth.value = highestRight * factor.value
        canvasHeight.value = highestBottom * factor.value
      }
    }

    onMounted(async () => {
      updateCanvas()

      listener.value = await listen<ICursorPosition>(CURSOR_POSITION, (event) => {
        cursorX.value = event.payload.x
        cursorY.value = event.payload.y
      })
    })

    return {
      state,
      displays,
      regions,
      canvasWidth,
      canvasHeight,
      topOffset,
      leftOffset,
      factor,
      updateCanvas,
      listener,
      cursorX,
      cursorY,
    }
  },
  updated() {
    this.updateCanvas()
  },
  unmounted() {
    if (this.listener) {
      this.listener()
    }
  },
})
</script>

<template>
  <div class="">
    <div
      class="relative"
      :style="{
        width: canvasWidth + 'px',
        height: canvasHeight + 'px',
      }"
      id="displayCanvas"
    >
      <div
        v-for="(display, index) in displays"
        :key="index"
        class="bg-white-rock-300 border-2 border-white-rock-600 absolute"
        :style="{
          top: display.top * factor + topOffset * factor + 'px',
          left: display.left * factor + leftOffset * factor + 'px',
          width: display.width * factor + 'px',
          height: display.height * factor + 'px',
        }"
      ></div>
      <div
        v-for="(region, index) in regions"
        :key="index"
        class="bg-rose-500/50 absolute"
        :style="{
          top: region.y * factor + topOffset * factor + 'px',
          left: region.x * factor + leftOffset * factor + 'px',
          width: region.width * factor + 'px',
          height: region.height * factor + 'px',
        }"
      ></div>
      <div
        class="w-2 h-2 rounded-full bg-white-rock-800 absolute"
        :style="{
          top: cursorY * factor + topOffset * factor - 4 + 'px',
          left: cursorX * factor + leftOffset * factor - 4 + 'px',
        }"
      ></div>
    </div>
  </div>
</template>
