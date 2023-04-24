import { invoke } from "./tauri"

export const connectToGrpc = async (hostname: string) => {
  await invoke("connect_grpc", { hostname })
}
