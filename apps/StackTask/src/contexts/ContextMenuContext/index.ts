import { createContext, useContext, type MouseEvent } from 'react'
import type { StoreApi } from 'zustand'
import { useStore } from 'zustand'

export type OpenState = {
  contextMenuId: string
  x: number
  y: number
}

export type ContextMenuStore = {
  readonly openState: OpenState | null
  createOnContextMenu: (contextMenuId: string) => (event: MouseEvent) => void
  close: () => void
}

export const ContextMenuContext = createContext<StoreApi<ContextMenuStore> | null>(null)

export const useContextMenuStore = () => {
  const store = useContext(ContextMenuContext)
  if (!store) {
    throw new Error('Missing ContextMenuProvider')
  }

  return useStore(store)
}
