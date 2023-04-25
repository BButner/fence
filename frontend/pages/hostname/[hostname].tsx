import { useAtom } from "jotai"
import { useRouter } from "next/router"
import { useEffect } from "react"

import { ConnectionStatusBar } from "@/components/connection/ConnectionStatusBar"
import { GrpcEventEmitter } from "@/components/EventHandler"
import { getDisplays } from "@/lib/displays-helper"
import { GrpcEvent } from "@/lib/events"
import { connectionStateAtom } from "@/lib/state/connection"
import { displaysAtom } from "@/lib/state/displays"
import { getCurrentState } from "@/lib/state/state"

export default function Hostname() {
  const router = useRouter()
  const { hostname } = router.query
  const [displays, setDisplays] = useAtom(displaysAtom)
  const [connectionState, setConnectionState] = useAtom(connectionStateAtom)

  useEffect(() => {
    void getCurrentState().then((state) => {
      if (!state.currentHostname) {
        void router.push("/")
      }
    })

    void getDisplays().then((displays) => {
      setDisplays(displays)
    })

    // GrpcEventEmitter.on(GrpcEvent.CONNECTION_LOST, () => {
    //   void router.push("/")
    // })
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  return (
    <div className="grid h-screen grid-flow-row grid-rows-[1fr_auto]">
      <div className="mx-auto my-8">
        {displays.map((display, idx) => (
          <div key={idx} className="mb-4">
            <div className="text-2xl font-bold">{`${display.width}x${display.height}`}</div>
            <div className="text-xl">{`x: ${display.left} y: ${display.top}`}</div>
          </div>
        ))}
      </div>
      <ConnectionStatusBar />
    </div>
  )
}
