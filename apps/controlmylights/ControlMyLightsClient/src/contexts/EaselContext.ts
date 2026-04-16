import type { Color } from '../types'
import { buildContext } from '../util/buildContext'

export type Splotch = {
  readonly color: Color
  readonly setColor: (color: Color) => void
  readonly active: boolean
  readonly setActive: () => void
}

export type EaselStore = {
  readonly splotches: Splotch[]
  readonly activeSplotch: Splotch | null
}

export type EaselProviderProps = {
  initialSplotchColors: Color[]
}

export const { StoreProvider: EaselProvider, useStoreContext: useEasel } = buildContext<
  EaselStore,
  EaselProviderProps
>((set, { initialSplotchColors }) => {
  const splotches = initialSplotchColors.map((color, index) => ({
    color,
    setColor: (color: Color) =>
      set(({ splotches }) => ({
        splotches: splotches.map((splotch, subIndex) => ({
          ...splotch,
          color: subIndex === index ? color : splotch.color,
        })),
      })),
    active: index === 0, // default the first splotch to active
    setActive: () =>
      set(({ splotches }) => {
        const newSplotches = splotches.map((splotch, subIndex) => ({
          ...splotch,
          active: index === subIndex,
        }))

        return {
          activeSplotch: newSplotches.find(({ active }) => active) ?? null,
          splotches: newSplotches,
        }
      }),
  }))

  return {
    activeSplotch: splotches.find(({ active }) => active) ?? null,
    splotches,
  }
})
