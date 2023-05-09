<script lang="ts">
import { defineComponent } from "vue"
import { IFenceState } from "~/lib/types/fence-state"
import { GrpcStatus } from "~/lib/types/grpc-status"

export default defineComponent({
  setup() {
    const state = inject<Ref<IFenceState | undefined>>("state")

    return {
      state,
      GrpcStatus,
    }
  },
})
</script>

<template>
  <div class="flex h-6 w-full items-center text-sm bg-white-rock-200">
    <button
      :class="{
        'flex h-6 items-center space-x-2 px-4 transition-all': true,
        'bg-twilight-blue-600': state?.grpcStatus === GrpcStatus.CONNECTED,
        'bg-mako-700':
          state?.grpcStatus === GrpcStatus.DISCONNECTED ||
          state?.grpcStatus === GrpcStatus.ERROR ||
          state?.grpcStatus === GrpcStatus.CONNECTION_LOST,
      }"
    >
      <font-awesome-icon
        v-if="state?.grpcStatus === GrpcStatus.CONNECTED"
        icon="fa-plug"
      />
      <font-awesome-icon
        v-if="state?.grpcStatus === GrpcStatus.CONNECTING"
        icon="fa-spinner"
      />
      <font-awesome-icon v-else icon="fa-plug-cirlcex" />
      <p>{{ state?.currentHostname }}</p>
    </button>
  </div>
</template>
