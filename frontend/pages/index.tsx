import { NewConnection } from "@/components/connection/NewConnection"
import { PreviousConnections } from "@/components/connection/PreviousConnections"

export default function Index() {
  return (
    <main className="grid grid-flow-col grid-cols-[320px_1fr]">
      <PreviousConnections />
      <NewConnection />
    </main>
  )
}
