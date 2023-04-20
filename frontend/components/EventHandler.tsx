import { UnlistenFn } from "@tauri-apps/api/event"
import { useAtom } from "jotai"
import { useEffect } from "react"

import { GrpcEvent, IEventPayload } from "@/lib/events"
import { connectionStateAtom, ConnectionStatus } from "@/lib/state/connection"
import { listen } from "@/lib/tauri"

export const EventHandler = () => {
  const [connectionState, setConnectionState] = useAtom(connectionStateAtom)

  useEffect(() => {
    const unlistenConnecting = listen<IEventPayload>(GrpcEvent.CONNECTING, (event) => {
      console.log("connecting", event)
      setConnectionState((state) => ({
        ...state,
        status: ConnectionStatus.CONNECTING,
      }))
    })

    const unlistenConnected = listen(GrpcEvent.CONNECTED, (event) => {
      console.log("connected", event)
      setConnectionState((state) => ({
        ...state,
        status: ConnectionStatus.CONNECTED,
      }))
    })

    const unlistenDisconnected = listen(GrpcEvent.DISCONNECTED, (event) => {
      console.log("disconnected", event)
      setConnectionState((state) => ({
        ...state,
        status: ConnectionStatus.DISCONNECTED,
      }))
    })

    const unlistenError = listen(GrpcEvent.ERROR, (event) => {
      console.log("error", event)
      setConnectionState((state) => ({
        ...state,
        status: ConnectionStatus.ERROR,
      }))
    })

    const unlistenConnectionLost = listen(GrpcEvent.CONNECTION_LOST, (event) => {
      console.log("connection lost", event)
      setConnectionState((state) => ({
        ...state,
        status: ConnectionStatus.CONNECTION_LOST,
      }))
    })

    return () => {
      void unlistenConnected.then((unlisten: void | UnlistenFn) => unlisten?.())
      void unlistenConnecting.then((unlisten: void | UnlistenFn) => unlisten?.())
      void unlistenDisconnected.then((unlisten: void | UnlistenFn) => unlisten?.())
      void unlistenError.then((unlisten: void | UnlistenFn) => unlisten?.())
      void unlistenConnectionLost.then((unlisten: void | UnlistenFn) => unlisten?.())
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])
  return <></>
}
