import type { Color } from '../types'
import { buildContext } from '../util/buildContext'

export type Splotch = {
  readonly color: Color
  readonly setColor: (color: Color) => void
  readonly active: boolean
  readonly setActive: () => void
}

export type PaletteStore = {
  readonly splotches: Splotch[]
  readonly activeSplotch: Splotch | null
}

export type PaletteProviderProps = {
  initialSplotchColors: Color[]
}

export const { StoreProvider: PaletteProvider, useStoreContext: usePalette } = buildContext<
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

  const setActiveSplotch = (index: number) =>
    set(({ splotches }) => {
      const newSplotches = splotches.map((splotch, subIndex) => ({
        ...splotch,
        active: index === subIndex,
      }))

      return {
        activeSplotch: newSplotches.find(({ active }) => active) ?? null,
        splotches: newSplotches,
      }
    })

  const splotches = initialSplotchColors.map((color, index) => ({
    color,
    setColor: (newColor: Color) => setSplotchColor(index, newColor),
    active: index === 0, // default the first splotch to active
    setActive: () => setActiveSplotch(index),
  }))

  return {
    activeSplotch: splotches.find(({ active }) => active) ?? null,
    splotches,
  }
})
