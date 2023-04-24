import "@/styles/globals.scss"

import { useAtom } from "jotai"
import type { AppProps } from "next/app"
import { useRouter } from "next/router"
import { useEffect } from "react"

import { Layout } from "@/components/Layout"
import { connectionStateAtom, ConnectionStatus } from "@/lib/state/connection"
import { currentStateAtom, getCurrentState } from "@/lib/state/state"

export default function App({ Component, pageProps }: AppProps) {
  const router = useRouter()
  const [currentState, setCurrentState] = useAtom(currentStateAtom)
  const [connectionState, setConnectionState] = useAtom(connectionStateAtom)

  useEffect(() => {
    void getCurrentState().then((state) => {
      console.log("_app.tsx: ", state)
      setCurrentState(state)
      console.log(ConnectionStatus[state.grpcStatus as keyof typeof ConnectionStatus])

      setConnectionState({
        ...connectionState,
        hostname: state.currentHostname,
        status: ConnectionStatus[state.grpcStatus as keyof typeof ConnectionStatus],
      })

      if (state.currentHostname) {
        const encodedHostname = encodeURIComponent(state.currentHostname)
        void router.push(`/hostname/${encodedHostname}`)
      }
    })

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])
  return (
    <Layout>
      <Component {...pageProps} />
    </Layout>
  )
}
