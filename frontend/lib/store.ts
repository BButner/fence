import { atom } from "jotai"

import { COMMAND_CONNECT_GRPC, COMMAND_GET_DISPLAYS } from "./commands"
import { invoke } from "./tauri"
import { IDisplay } from "./types/displays"
import { IFenceState } from "./types/fence-state"
import { GrpcStatus } from "./types/grpc-status"
import { IRegion } from "./types/regions"

export class FenceClientStore {
  constructor(
    public stateAtom = atom<IFenceState>({
      currentHostname: undefined,
      grpcStatus: GrpcStatus.DISCONNECTED,
    }),
    public displaysAtom = atom<IDisplay[]>([]),
    public regionsAtom = atom<IRegion[]>([]),
  ) {}

  public connectGrpc = async (hostname: string) => {
    await invoke(COMMAND_CONNECT_GRPC, { hostname })
  }

  public refreshDisplays = async () => [
    await invoke<IDisplay[]>(COMMAND_GET_DISPLAYS).then((displays) =>
      this.displays.write(displays),
    ),
  ]
}
