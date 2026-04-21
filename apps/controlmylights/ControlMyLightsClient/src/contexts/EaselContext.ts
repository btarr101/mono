import type { Color } from '../types'
import { buildContext } from '../util/buildContext'

export type Led = {
  readonly color: Color
  readonly setColor: (color: Color) => void
}

export type EaselStore = {
  readonly leds: Led[]
}

export type EaselProviderProps = {
  initialLedAttributes: {
    color: Color
  }[]
}

export const { StoreProvider: EaselProvider, useStoreContext: useEasel } = buildContext<
  EaselStore,
  EaselProviderProps
>((set, _, { initialLedAttributes: ledAttributes }) => ({
  leds: ledAttributes.map(({ color }, index) => ({
    color,
    setColor: color =>
      set(({ leds }) => ({
        leds: leds.map((led, subIndex) => ({
          ...led,
          color: subIndex === index ? color : led.color,
        })),
      })),
  })),
}))
