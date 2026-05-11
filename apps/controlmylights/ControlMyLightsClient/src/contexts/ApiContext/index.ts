import { createContext, useContext } from 'react'

import type { Color } from '../../types'

export type Led = {
  color: Color
  lastUpdated: Date
}

export type ApiStore = {
  latestLeds?: Led[]
  sendLedUpdate: (id: number, color: Color) => void
}

export const ApiContext = createContext<ApiStore | null>(null)

export const useApi = () => {
  const store = useContext(ApiContext)
  if (!store) {
    throw new Error('Missing ApiProvider')
  }

  return store
}
