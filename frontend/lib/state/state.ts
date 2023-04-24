import { atom } from "jotai"

import { GrpcEvent } from "../events"
import { invoke } from "../tauri"

export interface ICurrentState {
  currentHostname?: string
  grpcStatus: string
}

export const currentStateAtom = atom<ICurrentState>({
  grpcStatus: GrpcEvent.DISCONNECTED,
})

export const getCurrentState = async (): Promise<ICurrentState> => {
  return await invoke<ICurrentState>("get_state").then((data) => {
    return data
  })
}
