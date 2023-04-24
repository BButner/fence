import { useAtom } from "jotai"
import { useRouter } from "next/router"
import { useEffect } from "react"

import { ConnectionStatusBar } from "@/components/connection/ConnectionStatusBar"
import { GrpcEventEmitter } from "@/components/EventHandler"
import { GrpcEvent } from "@/lib/events"
import { connectionStateAtom } from "@/lib/state/connection"
import { getCurrentState } from "@/lib/state/state"

export default function Hostname() {
  const router = useRouter()
  const { hostname } = router.query

  const [connectionState, setConnectionState] = useAtom(connectionStateAtom)

  useEffect(() => {
    void getCurrentState().then((state) => {
      if (!state.currentHostname) {
        void router.push("/")
      }
    })

    // GrpcEventEmitter.on(GrpcEvent.CONNECTION_LOST, () => {
    //   void router.push("/")
    // })
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  return (
    <div className="grid h-screen grid-flow-row grid-rows-[1fr_auto]">
      <div className="">{hostname}</div>
      <ConnectionStatusBar />
    </div>
  )
}
