import { useEffect, useState } from "react"

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
    <div className="flex items-center justify-center">
      <p>The Beginning...</p>
      <input
        type="text"
        value={hostname}
        onChange={(e) => setHostname(e.currentTarget.value)}
      />
      <button onClick={() => void testConnection()}>Test Connection</button>
      <button onClick={() => void logState()}>Log State</button>
    </div>
  )
}
