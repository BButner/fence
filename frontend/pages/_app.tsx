import "@/styles/globals.scss"

import { useAtom } from "jotai"
import type { AppProps } from "next/app"
import { useRouter } from "next/router"
import { useEffect } from "react"

import { Layout } from "@/components/Layout"
import { currentStateAtom, getCurrentState } from "@/lib/state/state"

export default function App({ Component, pageProps }: AppProps) {
  const router = useRouter()
  const [currentState, setCurrentState] = useAtom(currentStateAtom)

  useEffect(() => {
    void getCurrentState().then((state) => {
      console.log("_app.tsx: ", state)
      setCurrentState(state)

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
