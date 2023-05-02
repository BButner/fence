import "@/styles/globals.scss"

import { useAtom } from "jotai"
import type { AppProps } from "next/app"
import { useEffect } from "react"

import { FenceApi } from "@/lib/api"
import { displayStateAtom, fenceStateAtom, regionStateAtom } from "@/lib/store"
import { listen } from "@/lib/tauri"
import { GRPC_STATUS } from "@/lib/types/events"
import { GrpcStatus, IGrpcEventPayload } from "@/lib/types/grpc-status"

let loaded = false

export default function App({ Component, pageProps }: AppProps) {
  const [, setState] = useAtom(fenceStateAtom)
  const [, setDisplays] = useAtom(displayStateAtom)
  const [, setRegions] = useAtom(regionStateAtom)

  useEffect(() => {
    if (loaded) return

    if (!loaded) {
      loaded = true
    }

    FenceApi.getState()
      .then(async (state) => {
        setState(state)

        // const displays = await FenceApi.getDisplays()
        // setDisplays(displays)

        const regions = await FenceApi.getRegions()
        setRegions(regions)
      })
      .catch((err) => {
        console.error(err)
      })

    const unlisten = listen<IGrpcEventPayload>(GRPC_STATUS, (event) => {
      setState((currentState) => {
        const newState = { ...currentState }
        newState.grpcStatus = GrpcStatus[event.payload.event as keyof typeof GrpcStatus]
        return newState
      })
    })

    return () => {
      void unlisten.then((u) => {
        if (u) {
          u()
        }
      })
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  return <Component {...pageProps} />
}
