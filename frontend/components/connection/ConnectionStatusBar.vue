<script lang="ts">
import { defineComponent } from "vue"
import { FenceApi } from "~/lib/api"
import { IFenceState } from "~/lib/types/fence-state"
import { GrpcStatus } from "~/lib/types/grpc-status"

export default defineComponent({
  setup() {
    const state = inject<Ref<IFenceState | undefined>>("state")

    const handleButtonClick = () => {
      if (state?.value) {
        if (
          [
            GrpcStatus.CONNECTION_LOST,
            GrpcStatus.DISCONNECTED,
            GrpcStatus.ERROR,
          ].includes(state?.value?.grpcStatus)
        ) {
          if (state?.value?.currentHostname) {
            FenceApi.connectGrpc(state?.value?.currentHostname)
          }
        }
      }
    }

    return {
      handleButtonClick,
      state,
      GrpcStatus,
    }
  },
})
</script>

<template>
  <div class="flex h-6 w-full items-center text-sm bg-white-rock-200">
    <button
      @click="handleButtonClick"
      :class="{
        'flex h-6 items-center space-x-2 px-4 transition-all': true,
        'bg-twilight-blue-600': state?.grpcStatus === GrpcStatus.CONNECTED,
        'bg-mako-700':
          state?.grpcStatus === GrpcStatus.DISCONNECTED ||
          state?.grpcStatus === GrpcStatus.ERROR ||
          state?.grpcStatus === GrpcStatus.CONNECTION_LOST,
        'bg-tango-500': state?.grpcStatus === GrpcStatus.CONNECTING,
      }"
    >
      <div class="flex w-6 items-center">
        <font-awesome-icon
          v-if="state?.grpcStatus === GrpcStatus.CONNECTED"
          icon="fa-plug"
        />
        <font-awesome-icon
          v-if="state?.grpcStatus === GrpcStatus.CONNECTING"
          icon="fa-circle-notch"
          class="animate-spin"
        />
        <font-awesome-icon
          v-if="
            state &&
            [
              GrpcStatus.CONNECTION_LOST,
              GrpcStatus.DISCONNECTED,
              GrpcStatus.ERROR,
            ].includes(state?.grpcStatus)
          "
          icon="fa-plug-circle-xmark"
        />
      </div>
      <p>{{ state?.currentHostname }}</p>
    </button>
  </div>
</template>
