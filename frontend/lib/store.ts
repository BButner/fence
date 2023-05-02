import { atom } from "jotai"

import { IDisplay } from "./types/displays"
import { IFenceState } from "./types/fence-state"
import { GrpcStatus } from "./types/grpc-status"
import { IRegion } from "./types/regions"

export const fenceStateAtom = atom<IFenceState>({
  currentHostname: undefined,
  grpcStatus: GrpcStatus.DISCONNECTED,
})

export const regionStateAtom = atom<IRegion[]>([])

export const displayStateAtom = atom<IDisplay[]>([])
