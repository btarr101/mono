import { zip } from 'lodash-es'

import type { Color } from '../types'
import { buildContext } from '../util/buildContext'
import { buildUseStoreState, buildUseStoreSubscribe } from '../util/buildContext/util'

export type Led = {
  color: Color
  lastUpdated?: Date
}

export type EaselStore = {
  leds: Led[]
  setLed: (index: number, color: Color) => void
  setLeds: (colors: Color[]) => void
  // subscribeToLedColorUpdated: (listener: (index: number, color: Color) => void) => () => void
}

export type EaselProviderProps = {
  initialColors: Color[]
}

export type LedColorUpdatedListener = (index: number, color: Color) => void

const { StoreProvider: EaselProvider, useStoreApi: useEaselApi } = buildContext<
  EaselStore,
  EaselProviderProps
>((set, _, { initialColors }) => ({
  leds: initialColors.map(color => ({
    color,
  })),
  setLed: (index: number, color: Color) =>
    set(({ leds }) => ({
      leds: leds.map((led, subIndex) => ({
        ...led,
        color: subIndex === index ? color : led.color,
      })),
    })),
  setLeds: colors =>
    set(() => {
      const now = new Date()

      return {
        leds: colors.map(color => ({
          color,
          lastUpdated: now,
        })),
      }
    }),
}))

const useEasel = buildUseStoreState(useEaselApi)

const useEaselSubscribe = buildUseStoreSubscribe(useEaselApi)

const useEaselLedColorUpdated = (listener: LedColorUpdatedListener) =>
  useEaselSubscribe(({ leds }, { leds: prevLeds }) =>
    zip(leds, prevLeds).map(([led, prevLed], index) => {
      if (!led) return
      if (
        led.color.red === prevLed?.color.red &&
        led.color.green === prevLed?.color.green &&
        led.color.blue === prevLed?.color.blue
      )
        return

      listener(index, led.color)
    }),
  )

export { EaselProvider, useEasel, useEaselLedColorUpdated }
