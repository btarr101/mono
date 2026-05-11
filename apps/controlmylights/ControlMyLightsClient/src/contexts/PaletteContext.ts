import type { Color } from '../types'
import { buildContext } from '../util/buildContext'
import { buildUseStoreState } from '../util/buildContext/util'

export type Splotch = {
  color: Color
}

export type PaletteStore = {
  splotches: Splotch[]
  activeSplotchIndex?: number
  setSplotchColor: (index: number, color: Color) => void
  setActiveSplotchIndex: (index: number) => void
}

export type PaletteProviderProps = {
  initialSplotchColors: Color[]
}

const { StoreProvider: PaletteProvider, useStoreApi: usePaletteApi } = buildContext<
  PaletteStore,
  PaletteProviderProps
>((set, _, { initialSplotchColors }) => {
  const setSplotchColor = (index: number, color: Color) =>
    set(({ splotches }) => ({
      splotches: splotches.map((splotch, subIndex) => ({
        ...splotch,
        color: index === subIndex ? color : splotch.color,
      })),
    }))

  const setActiveSplotchIndex = (index: number) => set({ activeSplotchIndex: index })

  const splotches = initialSplotchColors.map(color => ({
    color,
  }))

  return {
    activeSplotchIndex: 0,
    setActiveSplotchIndex,
    setSplotchColor,
    splotches,
  }
})

const usePalette = buildUseStoreState(usePaletteApi)

export { PaletteProvider, usePalette }

export const usePaletteSplotches = () => usePalette(state => state.splotches)

export const usePaletteActiveSplotch = () => {
  const activeSplotchIndex = usePalette(state => state.activeSplotchIndex)
  const splotches = usePaletteSplotches()

  const activeSplotch = activeSplotchIndex !== undefined ? splotches[activeSplotchIndex] : undefined

  return {
    activeSplotchIndex,
    activeSplotch,
  }
}

export const usePaletteActions = () => {
  const setActiveSpotchIndex = usePalette(state => state.setActiveSplotchIndex)
  const setSplotchColor = usePalette(state => state.setSplotchColor)

  return {
    setActiveSpotchIndex,
    setSplotchColor,
  }
}
