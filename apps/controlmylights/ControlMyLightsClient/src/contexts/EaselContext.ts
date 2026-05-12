import { zip } from 'lodash-es'

import type { Color, Led } from '../types'
import { buildContext } from '../util/buildContext'
import { buildUseStoreState, buildUseStoreSubscribe } from '../util/buildContext/util'

export type EaselStore = {
  leds: Led[]
  setLed: (index: number, color: Color) => void
  updateLeds: (newLeds: Led[]) => void
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
  setLed: (index: number, color: Color) => {
    const now = new Date()

    set(({ leds }) => ({
      leds: leds.map((led, subIndex) => ({
        ...led,
        ...(subIndex !== index
          ? {}
          : {
              color,
              lastUpdated: {
                timestamp: now,
                source: 'client',
              },
            }),
      })),
    }))
  },
  updateLeds: leds => {
    const nowTime = new Date().getTime()

    set(({ leds: previousLeds }) => ({
      leds: zip(previousLeds, leds).flatMap(([previousLed, led]) => {
        // Return the previous led if current leds are not set
        if (!led) {
          if (!previousLed) return []
          return previousLed
        }

        // Return new led if previous led lacks timestamp
        if (!previousLed?.lastUpdated) return led

        // Return previous led if current led lacks timestamp (though this should not happen likely)
        if (!led.lastUpdated) return previousLed

        // For eventual consistency, we give client leds an expiration of 1 second
        //
        // This leads to temporary desyncs between client and server, but then eventually when an update comes
        // if the server led is different from the client and it's been over a second - we assume the client
        // is in the wrong and needs to fix their shit
        //
        // Note: A further optimization would be polling the server if we have ANY client leds - but I think this case
        // is going to be rare and barely noticeable - wouldn't be mentally difficult to add just a PITA
        if (
          previousLed.lastUpdated.source === 'client' &&
          nowTime - previousLed.lastUpdated.timestamp.getTime() > 1000 // 1 second buffer to make the round trip
        ) {
          return led
        }

        // Return the latest led (leaning towards the new one)
        return led.lastUpdated.timestamp > previousLed.lastUpdated.timestamp ? led : previousLed
      }),
    }))
  },
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
