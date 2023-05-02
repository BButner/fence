import { ConnectionStatusBar } from "@/components/connection/ConnectionStatusBar"
import { DisplaysCanvas } from "@/components/displays/DisplaysCanvas"

export default function Hostname() {
  return (
    <div className="grid h-screen grid-flow-row grid-rows-[1fr_auto]">
      <div className="mx-auto">
        <DisplaysCanvas />
      </div>
      <ConnectionStatusBar />
    </div>
  )
}
