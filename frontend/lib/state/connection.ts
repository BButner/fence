import { atom } from "jotai"

export enum ConnectionStatus {
  CONNECTING = "CONNECTING",
  CONNECTED = "CONNECTED",
  DISCONNECTED = "DISCONNECTED",
  ERROR = "ERROR",
  CONNECTION_LOST = "CONNECTION_LOST",
  HEARTBEAT = "HEARTBEAT",
}

interface ConnectionState {
  status: ConnectionStatus
  hostname?: string
}

export const connectionStateAtom = atom<ConnectionState>({
  status: ConnectionStatus.DISCONNECTED,
})
