import { useEffect, useState } from "react"

import { NewConnection } from "@/components/connection/NewConnection"
import { PreviousConnections } from "@/components/connection/PreviousConnections"
import { invoke } from "@/lib/tauri"

export default function Home() {
  const [hostname, setHostname] = useState("")

  const logState = async () => {
    console.log("getting state...")
    await invoke("get_state").then((res) => console.log(res))
  }

  const testConnection = async () => {
    await invoke("connect_grpc", { hostname }).then((res) => console.log(res))
  }

  return (
    <div className="grid grid-flow-col grid-cols-[320px_1fr]">
      <PreviousConnections />
      <NewConnection />
    </div>
  )
}
