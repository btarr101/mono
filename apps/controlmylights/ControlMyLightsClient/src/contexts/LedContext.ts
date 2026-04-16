import type { Color } from '../types'
import { buildContext } from '../util/buildContext'

export type Led = {
  readonly color: Color
  readonly setColor: (color: Color) => void
}

export type LedStore = {
  readonly leds: Led[]
}

export type LedProviderProps = {
  ledAttributes: {
    color: Color
  }[]
}

export const { StoreProvider: LedProvider, useStoreContext: useLeds } = buildContext<
  LedStore,
  LedProviderProps
>((set, { ledAttributes }) => ({
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
