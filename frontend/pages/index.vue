<script lang="ts">
import { defineComponent } from "vue"
import { FenceApi } from "~/lib/api"
import { IDisplay } from "~/lib/types/displays"
import { IFenceState } from "~/lib/types/fence-state"
import { IRegion } from "~/lib/types/regions"

export default defineComponent({
  data() {
    return {
      hostname: "",
    }
  },
  setup() {
    const connectGrpc = (hostname: string) => {
      console.log("connecting to", hostname)
      FenceApi.connectGrpc(hostname)
        .then(() => {
          console.log("connected")
        })
        .catch((err) => {
          console.log(err)
        })
    }

    const state = inject<IFenceState>("state")
    const displays = inject<IDisplay[]>("displays")
    const regions = inject<IRegion[]>("regions")

    return {
      connectGrpc,
      state,
      displays,
      regions,
    }
  },
})
</script>

<template>
  <div class="w-full h-screen flex items-center justify-center">
    <div class="flex items-end space-x-4">
      <div class="">
        <label class="block font-semibold" for="connecting-hostname">Hostname</label>
        <input
          class="text-3xl px-2 h-14 shadow-lg"
          type="text"
          id="connecting-hostname"
          v-model="hostname"
        />
      </div>
      <button
        @click="connectGrpc(hostname)"
        class="bg-tango-500 px-8 h-14 text-xl font-semibold text-white-rock-50 shadow-lg"
      >
        Connect
      </button>
    </div>
    <EventHandler />
  </div>
</template>
