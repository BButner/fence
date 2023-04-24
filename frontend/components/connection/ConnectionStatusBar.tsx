import { faPlug, faPlugCircleXmark, faSpinner } from "@fortawesome/free-solid-svg-icons"
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome"
import clsx from "clsx"
import { useAtomValue } from "jotai"

import { connectToGrpc } from "@/lib/connection-helper"
import { connectionStateAtom, ConnectionStatus } from "@/lib/state/connection"

export const ConnectionStatusBar: React.FC = () => {
  const connectionState = useAtomValue(connectionStateAtom)

  const onClick = async () => {
    if (
      [ConnectionStatus.CONNECTION_LOST, ConnectionStatus.DISCONNECTED].includes(
        connectionState.status,
      )
    ) {
      if (connectionState.hostname) {
        await connectToGrpc(connectionState.hostname)
      }
    }
  }

  return (
    <div className="flex h-6 w-screen bg-gray-300 text-sm">
      <button
        onClick={() => void onClick()}
        className={clsx(
          "flex items-center space-x-4 px-4 transition-all",
          connectionState.status === ConnectionStatus.CONNECTED &&
            "bg-green-400/50 text-green-900",
          connectionState.status === ConnectionStatus.CONNECTING &&
            "bg-yellow-400/50 text-yellow-900",
          (connectionState.status === ConnectionStatus.DISCONNECTED ||
            connectionState.status === ConnectionStatus.CONNECTION_LOST) &&
            "bg-red-400/50 text-red-900",
        )}
      >
        <div className="w-6">
          {connectionState.status === ConnectionStatus.CONNECTED && (
            <FontAwesomeIcon icon={faPlug} />
          )}
          {connectionState.status === ConnectionStatus.CONNECTING && (
            <FontAwesomeIcon icon={faSpinner} className="animate-spin" />
          )}
          {[ConnectionStatus.CONNECTION_LOST, ConnectionStatus.DISCONNECTED].includes(
            connectionState.status,
          ) && <FontAwesomeIcon icon={faPlugCircleXmark} className="animate-pulse" />}
        </div>
        <span>{connectionState.hostname}</span>
      </button>
    </div>
  )
}
