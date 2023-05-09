import { invoke } from "@tauri-apps/api"

import { IDisplay, IDisplayScreenshot } from "./types/displays"
import { IFenceState } from "./types/fence-state"
import { IRegion } from "./types/regions"
import {
  COMMAND_CONNECT_GRPC,
  COMMAND_GET_DISPLAYS,
  COMMAND_GET_REGIONS,
  COMMAND_GET_STATE,
  COMMAND_GET_DISPLAY_SCREENSHOTS,
} from "./commands"
import { GrpcStatus } from "./types/grpc-status"

export abstract class FenceApi {
  public static connectGrpc = async (hostname: string): Promise<void> => {
    return await invoke<void>(COMMAND_CONNECT_GRPC, { hostname })
  }

  public static getState = async (): Promise<IFenceState> => {
    let state = await invoke<IFenceState>(COMMAND_GET_STATE)
    console.log(state)
    state.grpcStatus = GrpcStatus[state.grpcStatus as keyof typeof GrpcStatus]
    return state
  }

  public static getDisplays = async (): Promise<IDisplay[]> => {
    return await invoke<IDisplay[]>(COMMAND_GET_DISPLAYS)
  }

  public static getRegions = async (): Promise<IRegion[]> => {
    return await invoke<IRegion[]>(COMMAND_GET_REGIONS)
  }

  public static getScreenshots = async (): Promise<IDisplayScreenshot[]> => {
    return await invoke<IDisplayScreenshot[]>(COMMAND_GET_DISPLAY_SCREENSHOTS)
  }
}
