import { createStore, useStore } from 'zustand'
import { ContextMenuContext, type ContextMenuStore } from '.'
import { useEffect, useState, type MouseEvent, type PropsWithChildren } from 'react'

const buildContextMenuStore = () =>
  createStore<ContextMenuStore>(set => ({
    openState: null,
    createOnContextMenu: contextMenuId => (event: MouseEvent) => {
      event.preventDefault()
      set({
        openState: {
          contextMenuId,
          x: event.clientX,
          y: event.clientY,
        },
      })
    },
    close: () => set({ openState: null }),
  }))

export const ContextMenuProvider = ({ children }: PropsWithChildren) => {
  const [store] = useState(buildContextMenuStore)
  const { openState, close } = useStore(store)

  useEffect(() => {
    if (!openState) return

    const onKey = (event: KeyboardEvent) => {
      if (event.key === 'Escape') close()
    }
    window.addEventListener('click', close)
    window.addEventListener('scroll', close, true)
    window.addEventListener('keydown', onKey)

    return () => {
      window.removeEventListener('click', close)
      window.removeEventListener('scroll', close, true)
      window.removeEventListener('keydown', onKey)
    }
  }, [openState, close])

  return <ContextMenuContext.Provider value={store}>{children}</ContextMenuContext.Provider>
}
