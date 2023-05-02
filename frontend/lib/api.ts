import {
  COMMAND_CONNECT_GRPC,
  COMMAND_GET_DISPLAYS,
  COMMAND_GET_REGIONS,
  COMMAND_GET_STATE,
} from "./commands"
import { invoke } from "./tauri"
import { IDisplay } from "./types/displays"
import { IFenceState } from "./types/fence-state"
import { IRegion } from "./types/regions"

export abstract class FenceApi {
  public static connectGrpc = async (hostname: string): Promise<void> => {
    console.log(`api.ts: connectGrpc: hostname: ${hostname}`)
    return await invoke<void>(COMMAND_CONNECT_GRPC, { hostname })
  }

  public static getState = async (): Promise<IFenceState> => {
    return await invoke<IFenceState>(COMMAND_GET_STATE)
  }

  public static getDisplays = async (): Promise<IDisplay[]> => {
    return await invoke<IDisplay[]>(COMMAND_GET_DISPLAYS)
  }

  public static getRegions = async (): Promise<IRegion[]> => {
    return await invoke<IRegion[]>(COMMAND_GET_REGIONS)
  }
}
