import {
  faCircleNotch,
  faPlug,
  faPlugCircleXmark,
} from "@fortawesome/free-solid-svg-icons"
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome"
import clsx from "clsx"
import { useAtomValue } from "jotai"

import { FenceApi } from "@/lib/api"
import { fenceStateAtom } from "@/lib/store"
import { GrpcStatus } from "@/lib/types/grpc-status"

export const ConnectionStatusBar: React.FC = () => {
  const state = useAtomValue(fenceStateAtom)

  const statusButtonClick = () => {
    if (
      [GrpcStatus.CONNECTION_LOST, GrpcStatus.DISCONNECTED, GrpcStatus.ERROR].includes(
        state.grpcStatus,
      ) &&
      state.currentHostname
    ) {
      void FenceApi.connectGrpc(state.currentHostname)
    }
  }

  return (
    <div className="flex h-6 w-full items-center bg-gray-300 text-sm dark:bg-gray-800">
      <button
        onClick={statusButtonClick}
        className={clsx(
          "flex h-6 items-center space-x-2 px-4 transition-all",
          state.grpcStatus === GrpcStatus.CONNECTED &&
            "bg-green-400/50 text-green-900 dark:bg-green-400/20 dark:text-green-400",
          state.grpcStatus === GrpcStatus.CONNECTING &&
            "bg-yellow-400/50 text-yellow-900 dark:bg-yellow-400/20 dark:text-yellow-400",
          (state.grpcStatus === GrpcStatus.DISCONNECTED ||
            state.grpcStatus === GrpcStatus.CONNECTION_LOST ||
            state.grpcStatus === GrpcStatus.ERROR) &&
            "bg-red-400/50 text-red-900 dark:bg-red-400/20 dark:text-red-400",
        )}
      >
        <div className="flex w-6 items-center">
          {state.grpcStatus === GrpcStatus.CONNECTED && (
            <FontAwesomeIcon icon={faPlug} />
          )}
          {state.grpcStatus === GrpcStatus.CONNECTING && (
            <FontAwesomeIcon
              icon={faCircleNotch}
              className="m-0 h-4 origin-center animate-spin p-0"
            />
          )}
          {[
            GrpcStatus.CONNECTION_LOST,
            GrpcStatus.DISCONNECTED,
            GrpcStatus.ERROR,
          ].includes(state.grpcStatus) && (
            <FontAwesomeIcon icon={faPlugCircleXmark} className="animate-pulse" />
          )}
        </div>
        <span>{state.currentHostname}</span>
      </button>
    </div>
  )
}
