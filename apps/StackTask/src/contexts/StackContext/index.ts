import { useStore, type StoreApi } from 'zustand'
import type { StackItem } from './types'
import { createContext, useContext } from 'react'

export type StackItemParams = Omit<StackItem, 'id'>

export type StackStore = {
  readonly items: readonly StackItem[]
  push: (item: StackItemParams) => void
  queue: (item: StackItemParams) => void
  pop: () => StackItem | null
  moveBefore: (id: string, id2: string) => void
  swap: (id: string, id2: string) => void
}

export const StackContext = createContext<StoreApi<StackStore> | null>(null)

export const useStackStore = () => {
  const store = useContext(StackContext)
  if (!store) {
    throw new Error('Missing StackProvider')
  }

  return useStore(store)
}
