<script lang="ts">
import { defineComponent } from "vue"
import { FenceApi } from "~/lib/api"
import { IDisplay } from "~/lib/types/displays"
import { IFenceState } from "~/lib/types/fence-state"
import { GrpcStatus } from "~/lib/types/grpc-status"
import { IRegion } from "~/lib/types/regions"

export default defineComponent({
  data() {
    return {
      hostname: "",
    }
  },
  setup() {
    const router = useRouter()

    const connectGrpc = (hostname: string) => {
      console.log("connecting to", hostname)
      FenceApi.connectGrpc(hostname)
        .then(() => {
          router.push(`/hostname/${encodeURIComponent(hostname)}`)
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
      GrpcStatus,
    }
  },
})
</script>

<template>
  <div
    class="w-full h-screen flex items-center justify-center relative overflow-hidden"
  >
    <div
      v-motion
      :initial="{
        opacity: 0,
        scale: 0.5,
        y: 100,
      }"
      :enter="{
        opacity: 1,
        scale: 1,
        y: 0,
      }"
      class="z-10"
    >
      <h1
        class="bg-gradient-to-br from-tango-500 to-tango-600 bg-clip-text fill-transparent text-center text-8xl font-semibold tracking-wide text-transparent mb-16"
      >
        Fence
      </h1>
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
          class="bg-tango-500 h-14 w-14 text-xl font-semibold text-white-rock-50 shadow-lg flex items-center space-x-2 justify-center"
        >
          <div class="w-14 h-14 flex items-center justify-center">
            <font-awesome-icon
              icon="fa-plug"
              class="text-xl"
              v-if="
                state?.grpcStatus === GrpcStatus.DISCONNECTED ||
                state?.grpcStatus === GrpcStatus.ERROR
              "
            />
            <font-awesome-icon
              icon="fa-circle-notch"
              class="text-xl animate-spin"
              v-else
            />
          </div>
        </button>
      </div>
    </div>

    <div
      v-motion
      :initial="{
        transform: 'translateX(-100%) rotate(0deg)',
      }"
      :enter="{
        transform: 'translateX(-50%) rotate(45deg)',
      }"
      className="absolute left-0 top-0 z-0 h-96 w-96 bg-mako-400/80 transition-colors hover:bg-twilight-blue-500/50"
    />
    <div
      v-motion
      :initial="{
        transform: 'translateX(100%) rotate(0deg)',
      }"
      :enter="{
        transform: 'translateX(60%) rotate(-45deg)',
      }"
      className="absolute bottom-0 right-0 z-0 h-96 w-96 animate-rotate-negative-45 bg-mako-400/80 transition-colors hover:bg-twilight-blue-500/50"
    />
  </div>
</template>

<style lang="scss"></style>
