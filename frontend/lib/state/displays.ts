import { atom } from "jotai"

import { IDisplay } from "../displays-helper"

export const displaysAtom = atom<IDisplay[]>([])
