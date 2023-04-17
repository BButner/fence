import { UnlistenFn } from "@tauri-apps/api/event"
import { useEffect } from "react"

import { GrpcEvent } from "@/lib/events"
import { listen } from "@/lib/tauri"

export const EventHandler = () => {
  useEffect(() => {
    const unlistenConnecting = listen(GrpcEvent.CONNECTING, (event) => {
      console.log("connecting", event)
    })

    const unlistenConnected = listen(GrpcEvent.CONNECTED, (event) => {
      console.log("connected", event)
    })

    const unlistenDisconnected = listen(GrpcEvent.DISCONNECTED, (event) => {
      console.log("disconnected", event)
    })

    const unlistenError = listen(GrpcEvent.ERROR, (event) => {
      console.log("error", event)
    })

    const unlistenConnectionLost = listen(GrpcEvent.CONNECTION_LOST, (event) => {
      console.log("connection lost", event)
    })

    return () => {
      void unlistenConnected.then((unlisten: void | UnlistenFn) => unlisten?.())
      void unlistenConnecting.then((unlisten: void | UnlistenFn) => unlisten?.())
      void unlistenDisconnected.then((unlisten: void | UnlistenFn) => unlisten?.())
      void unlistenError.then((unlisten: void | UnlistenFn) => unlisten?.())
      void unlistenConnectionLost.then((unlisten: void | UnlistenFn) => unlisten?.())
    }
  }, [])
  return <></>
}
