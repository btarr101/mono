import { useState, type PropsWithChildren } from 'react'
import { createStore } from 'zustand'
import { StackContext, type StackStore } from '.'
import { v4 } from 'uuid'

const buildStackStore = () =>
  createStore<StackStore>(set => ({
    items: [],
    push: item => {
      set(({ items }) => ({ items: [...items, { id: v4(), ...item }] }))
    },
    queue: item => {
      set(({ items }) => ({ items: [{ id: v4(), ...item }, ...items] }))
    },
    pop: () => {
      let last = null

      set(({ items }) => {
        if (items.length === 0) return { items }
        last = items[items.length - 1]

        return { stack: items.slice(0, -1) }
      })

      return last
    },
    moveBefore: (id, beforeId) => {
      set(({ items }) => {
        const from = items.findIndex(item => item.id === id)
        if (from === -1) return { items }

        const newItems = items.slice()
        const [moving] = newItems.splice(from, 1)

        if (beforeId === null) {
          newItems.push(moving!)
          return {
            items: newItems,
          }
        }

        const to = items.findIndex(item => item.id === beforeId)
        if (to === -1) return { items }

        const insertAt = from < to ? to - 1 : to
        newItems.splice(insertAt, 0, moving!)

        return {
          items: newItems,
        }
      })
    },
  }))

export const StackProvider = ({ children }: PropsWithChildren) => {
  const [store] = useState(buildStackStore)

  return <StackContext.Provider value={store}>{children}</StackContext.Provider>
}
