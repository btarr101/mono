import { chunk } from 'lodash-es'
import { type PropsWithChildren, useCallback, useEffect, useState } from 'react'
import { useWebSocket } from 'react-use-websocket/dist/lib/use-websocket.js'
import { createStore } from 'zustand'

import type { Color, Led } from '../../types'
import { ApiContext, type LatestLedsStore } from '.'

export const ApiProvider = ({ children }: PropsWithChildren) => {
  const [latestLedsStore] = useState(() => createStore<LatestLedsStore>()(() => ({})))

  const { sendMessage, lastMessage } = useWebSocket(
    `${import.meta.env.VITE_API_BASE_URL ?? '/api'}/leds/ws`,
    {
      disableJson: true,
      retryOnError: true,
      reconnectAttempts: 20,
      reconnectInterval: attemptNumber => Math.min(1000 * 2 ** attemptNumber, 10000),
      shouldReconnect: () => true,
      heartbeat: {
        interval: 60000,
        message: 'ping',
        returnMessage: 'pong',
      },
    },
  )

  const sendLedUpdate = useCallback(
    (id: number, color: Color) => {
      // ------------------------------------------------------
      // |  Byte 0 - Byte 1  |  Byte 2  |  Byte 3  |  Byte 4  |
      // ------------------------------------------------------
      // |       Index       |    R     |    G     |    B     |
      // ------------------------------------------------------
      const { red, green, blue } = color
      const low = id & 0xff
      const high = (id >> 8) & 0xff
      sendMessage(new Uint8Array([high, low, red, green, blue]))
    },
    [sendMessage],
  )

  useEffect(() => {
    if (!lastMessage?.data) return
    ;(async () => {
      const buffer = await (lastMessage.data as Blob).arrayBuffer()
      const array = new Uint8Array(buffer)

      const nextLeds = chunk(array, 11).map(([red, green, blue, ...timestampBytes]) => {
        // ---------------------------------------------------------
        // |  Byte 0  |  Byte 1  |  Byte 2  |   Byte 3 - Byte 10   |
        // ---------------------------------------------------------
        // |    R     |    G     |    B     |      Timestamp       |
        // ---------------------------------------------------------
        const dataView = new DataView(new Uint8Array(timestampBytes).buffer)
        const timestamp = new Date(Number(dataView.getBigUint64(0, false)) * 1000)
        return {
          color: { red: red ?? 0, green: green ?? 0, blue: blue ?? 0 },
          lastUpdated: {
            source: 'server',
            timestamp,
          },
        } satisfies Led
      })

      latestLedsStore.setState({
        leds: nextLeds,
      })
    })()
  }, [latestLedsStore, lastMessage])

  return (
    <ApiContext.Provider
      value={{
        latestLedsStore,
        sendLedUpdate,
      }}
    >
      {children}
    </ApiContext.Provider>
  )
}
