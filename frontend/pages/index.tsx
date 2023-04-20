import { NewConnection } from "@/components/connection/NewConnection"
import { PreviousConnections } from "@/components/connection/PreviousConnections"

export default function Home() {
  return (
    <div className="grid grid-flow-col grid-cols-[320px_1fr]">
      <PreviousConnections />
      <NewConnection />
    </div>
  )
}
