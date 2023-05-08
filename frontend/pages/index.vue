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
  <div>
    <h1>Home page</h1>
    <p>{{ hostname }}</p>
    <p>{{ state?.grpcStatus }}</p>
    <p>{{ displays?.length }}</p>
    <p>{{ regions?.length }}</p>
    <input type="text" v-model="hostname" />
    <button @click="connectGrpc(hostname)">Connect</button>
  </div>
</template>
