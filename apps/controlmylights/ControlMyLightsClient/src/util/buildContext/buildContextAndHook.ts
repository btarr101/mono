import { createContext, useContext } from 'react'
import { type StoreApi } from 'zustand'

export const buildContextAndHooks = <Store>() => {
  const Context = createContext<StoreApi<Store> | null>(null)

  const useStoreApi = () => {
    const store = useContext(Context)
    if (!store) {
      throw new Error(`Missing ${Context.name}`)
    }

    return store
  }

  return {
    Context,
    useStoreApi,
  }
}
