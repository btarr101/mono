import { createContext, useContext } from 'react'
import { type StoreApi, useStore } from 'zustand'

export const buildContextAndHook = <Store>() => {
  const Context = createContext<StoreApi<Store> | null>(null)
  const useStoreContext = () => {
    const store = useContext(Context)
    if (!store) {
      throw new Error(`Missing ${Context.name}`)
    }

    return useStore(store)
  }

  return {
    Context,
    useStoreContext,
  }
}
