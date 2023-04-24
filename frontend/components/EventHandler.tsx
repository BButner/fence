import { UnlistenFn } from "@tauri-apps/api/event"
import { EventEmitter } from "events"
import { useAtom } from "jotai"
import { useEffect } from "react"

import { GrpcEvent, IEventPayload } from "@/lib/events"
import { connectionStateAtom, ConnectionStatus } from "@/lib/state/connection"
import { listen } from "@/lib/tauri"

export const GrpcEventEmitter = new EventEmitter()

export const EventHandler = () => {
  const [connectionState, setConnectionState] = useAtom(connectionStateAtom)

  useEffect(() => {
    const unlistenConnecting = listen<IEventPayload>(GrpcEvent.CONNECTING, (event) => {
      GrpcEventEmitter.emit(GrpcEvent.CONNECTING, event)
      setConnectionState((state) => ({
        ...state,
        status: ConnectionStatus.CONNECTING,
      }))
    })

    const unlistenConnected = listen(GrpcEvent.CONNECTED, (event) => {
      GrpcEventEmitter.emit(GrpcEvent.CONNECTED, event)
      setConnectionState((state) => ({
        ...state,
        status: ConnectionStatus.CONNECTED,
      }))
    })

    const unlistenDisconnected = listen(GrpcEvent.DISCONNECTED, (event) => {
      GrpcEventEmitter.emit(GrpcEvent.DISCONNECTED, event)
      setConnectionState((state) => ({
        ...state,
        status: ConnectionStatus.DISCONNECTED,
      }))
    })

    const unlistenError = listen(GrpcEvent.ERROR, (event) => {
      GrpcEventEmitter.emit(GrpcEvent.ERROR, event)
      setConnectionState((state) => ({
        ...state,
        status: ConnectionStatus.ERROR,
      }))
    })

    const unlistenConnectionLost = listen(GrpcEvent.CONNECTION_LOST, (event) => {
      GrpcEventEmitter.emit(GrpcEvent.CONNECTION_LOST, event)
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
