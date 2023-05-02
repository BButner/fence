import { faCircleNotch, faPlug } from "@fortawesome/free-solid-svg-icons"
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome"
import clsx from "clsx"
import { useAtomValue } from "jotai"
import { useRouter } from "next/router"
import { useState } from "react"

import { FenceApi } from "@/lib/api"
import { fenceStateAtom } from "@/lib/store"
import { GrpcStatus } from "@/lib/types/grpc-status"

export const NewConnection: React.FC = () => {
  const state = useAtomValue(fenceStateAtom)
  const [hostname, setHostname] = useState<string>("")
  const [connectionInvalid, setConnectionInvalid] = useState<boolean>(false)
  const router = useRouter()

  const attemptConnect = async () => {
    if (!hostname) {
      setConnectionInvalid(true)
      setInvalidTimeout()
      return
    }
    try {
      new URL(hostname)
    } catch (e) {
      setConnectionInvalid(false)
      setInvalidTimeout()
      return
    }

    await FenceApi.connectGrpc(hostname)
      .then(() => {
        const encodedUri = encodeURIComponent(hostname)
        void router.push(`/hostname/${encodedUri}`)
      })
      .catch(() => {
        setConnectionInvalid(true)
        setInvalidTimeout()
      })
  }
  const setInvalidTimeout = () => {
    setTimeout(() => {
      setConnectionInvalid(false)
    }, 2000)
  }

  return (
    <div className="relative flex h-full w-full items-center justify-center overflow-hidden">
      <div className="absolute left-0 top-0 z-0 h-96 w-96 -translate-x-1/2 rotate-45 bg-gray-300/60 transition-all hover:bg-red-300/50 dark:bg-gray-800/60 dark:hover:bg-red-600/30" />
      <div className="absolute bottom-0 right-0 z-0 h-96 w-96 translate-x-1/2 rotate-45 bg-gray-300/60 transition-all hover:bg-red-300/50 dark:bg-gray-800/50 dark:hover:bg-red-600/30" />
      <div className="z-10 space-y-12">
        <h1 className="bg-gradient-to-br from-violet-500 to-indigo-400 bg-clip-text fill-transparent text-center text-8xl font-semibold tracking-wide text-transparent">
          Fence
        </h1>
        <div className="flex items-center space-x-4">
          <input
            type="text"
            placeholder="Hostname"
            value={hostname}
            onChange={(e) => setHostname(e.target.value)}
            className={clsx(
              "h-14 w-96 rounded-lg border-4 border-indigo-500 px-2 text-2xl outline-none transition-all focus:ring-8 focus:ring-indigo-500/50 dark:bg-gray-950 dark:text-white",
              connectionInvalid &&
                "border-red-500/50 focus:ring-red-500/50 dark:border-red-500/50 dark:focus:ring-red-500/50",
            )}
          />
          <button
            onClick={() => void attemptConnect()}
            className={clsx(
              "flex h-14 w-44 items-center justify-between rounded-lg bg-indigo-500/20 px-8 text-xl text-indigo-800 outline-none transition-all hover:bg-indigo-500/30 focus:ring-8 focus:ring-indigo-500/50 dark:text-indigo-300",
              connectionInvalid &&
                "bg-red-500/20 text-red-800 hover:bg-red-500/40 focus:ring-red-500/50 dark:text-red-300",
            )}
          >
            {state.grpcStatus === GrpcStatus.CONNECTING ? (
              <span className="animate-spin">
                <FontAwesomeIcon icon={faCircleNotch} />
              </span>
            ) : (
              <FontAwesomeIcon icon={faPlug} />
            )}
            <p>Connect</p>
          </button>
        </div>
      </div>
    </div>
  )
}
