import { invoke } from "./tauri"

export interface IDisplay {
  width: number
  height: number
  top: number
  left: number
  isPrimary: boolean
}

export const getDisplays = async (): Promise<IDisplay[]> => {
  return await invoke<IDisplay[]>("get_displays")
    .then((displays) => displays)
    .catch((err) => {
      console.error(err)
      return []
    })
}
