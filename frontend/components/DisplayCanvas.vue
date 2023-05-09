<script lang="ts">
import { defineComponent } from "vue"
import { IDisplay } from "~/lib/types/displays"
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
    }
  },
  mounted() {
    this.updateCanvas()
  },
  updated() {
    this.updateCanvas()
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
    </div>
  </div>
</template>
