import "@/styles/globals.scss"

import { useAtom } from "jotai"
import type { AppProps } from "next/app"
import { useEffect } from "react"

import { FenceApi } from "@/lib/api"
import { fenceStateAtom } from "@/lib/store"
import { listen } from "@/lib/tauri"
import { GRPC_STATUS } from "@/lib/types/events"
import { GrpcStatus, IGrpcEventPayload } from "@/lib/types/grpc-status"

export default function App({ Component, pageProps }: AppProps) {
  const [, setState] = useAtom(fenceStateAtom)

  useEffect(() => {
    void FenceApi.getState().then(setState)

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
