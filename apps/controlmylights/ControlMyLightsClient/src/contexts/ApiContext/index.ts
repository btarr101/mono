import { createContext, useContext } from 'react'
import { type StoreApi, useStore } from 'zustand'

import type { Color, Led } from '../../types'

export type LatestLedsStore = {
  leds?: Led[]
}

export type ApiStore = {
  latestLedsStore: StoreApi<LatestLedsStore>
  sendLedUpdate: (id: number, color: Color) => void
}

export const ApiContext = createContext<ApiStore | null>(null)

export const useApi = () => {
  const store = useContext(ApiContext)
  if (!store) {
    throw new Error('Missing ApiProvider')
  }

  const latestLeds = useStore(store.latestLedsStore, state => state.leds)
  const sendLedUpdate = store.sendLedUpdate

  return {
    latestLeds,
    sendLedUpdate,
  }
}
