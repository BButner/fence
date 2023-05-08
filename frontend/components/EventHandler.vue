<script lang="ts">
import { IFenceState } from "~/lib/types/fence-state"
import { listen, UnlistenFn } from "@tauri-apps/api/event"
import { GRPC_STATUS } from "~/lib/types/events"
import { GrpcStatus, IGrpcEventPayload } from "~/lib/types/grpc-status"

export default defineComponent({
  data() {},
  setup() {
    let state = inject<Ref<IFenceState | undefined>>("state")
    let listener: undefined | UnlistenFn = undefined

    onMounted(async () => {
      listener = await listen<IGrpcEventPayload>(GRPC_STATUS, (event) => {
        console.log("received")
        if (state) {
          state.value = {
            ...state,
            grpcStatus: GrpcStatus[event.payload.event as keyof typeof GrpcStatus],
          }
        }
      })
    })

    onUnmounted(() => {
      if (listener) {
        listener()
      }
    })

    return {
      state,
    }
  },
})
</script>
