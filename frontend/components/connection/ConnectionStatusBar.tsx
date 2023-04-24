import {
  faCircleNotch,
  faPlug,
  faPlugCircleXmark,
} from "@fortawesome/free-solid-svg-icons"
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome"
import clsx from "clsx"
import { useAtomValue } from "jotai"

import { connectToGrpc } from "@/lib/connection-helper"
import { connectionStateAtom, ConnectionStatus } from "@/lib/state/connection"

export const ConnectionStatusBar: React.FC = () => {
  const connectionState = useAtomValue(connectionStateAtom)

  console.log(connectionState)

  const onClick = async () => {
    if (
      [
        ConnectionStatus.CONNECTION_LOST,
        ConnectionStatus.DISCONNECTED,
        ConnectionStatus.ERROR,
      ].includes(connectionState.status)
    ) {
      if (connectionState.hostname) {
        await connectToGrpc(connectionState.hostname)
      }
    }
  }

  return (
    <div className="flex h-6 w-screen items-center bg-gray-300 text-sm dark:bg-gray-800">
      <button
        onClick={() => void onClick()}
        className={clsx(
          "flex h-6 items-center space-x-2 px-4 transition-all",
          connectionState.status === ConnectionStatus.CONNECTED &&
            "bg-green-400/50 text-green-900 dark:bg-green-400/20 dark:text-green-400",
          connectionState.status === ConnectionStatus.CONNECTING &&
            "bg-yellow-400/50 text-yellow-900 dark:bg-yellow-400/20 dark:text-yellow-400",
          (connectionState.status === ConnectionStatus.DISCONNECTED ||
            connectionState.status === ConnectionStatus.CONNECTION_LOST ||
            connectionState.status === ConnectionStatus.ERROR) &&
            "bg-red-400/50 text-red-900 dark:bg-red-400/20 dark:text-red-400",
        )}
      >
        <div className="flex w-6 items-center">
          {connectionState.status === ConnectionStatus.CONNECTED && (
            <FontAwesomeIcon icon={faPlug} />
          )}
          {connectionState.status === ConnectionStatus.CONNECTING && (
            <FontAwesomeIcon
              icon={faCircleNotch}
              className="m-0 h-4 origin-center animate-spin p-0"
            />
          )}
          {[
            ConnectionStatus.CONNECTION_LOST,
            ConnectionStatus.DISCONNECTED,
            ConnectionStatus.ERROR,
          ].includes(connectionState.status) && (
            <FontAwesomeIcon icon={faPlugCircleXmark} className="animate-pulse" />
          )}
        </div>
        <span>{connectionState.hostname}</span>
      </button>
    </div>
  )
}
