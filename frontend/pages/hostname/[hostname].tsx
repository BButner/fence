import { useAtom } from "jotai"
import { useRouter } from "next/router"
import { useEffect } from "react"

import { connectionStateAtom } from "@/lib/state/connection"

export default function Hostname() {
  const router = useRouter()
  const { hostname } = router.query

  const [connectionState, setConnectionState] = useAtom(connectionStateAtom)

  useEffect(() => {
    if (!connectionState.hostname) {
      void router.push("/")
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  return <h1>Hostname: {hostname}</h1>
}
