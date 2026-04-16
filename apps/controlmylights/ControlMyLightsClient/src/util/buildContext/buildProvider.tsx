import { type Context, type PropsWithChildren, useState } from 'react'
import { createStore, type StateCreator, type StoreApi } from 'zustand'

type StoreSet<T> = Parameters<StateCreator<T>>[0]
export type EmptyProps = Record<string, never>

export type StoreBuilder<Store, ProviderProps = EmptyProps> = (
  set: StoreSet<Store>,
  providerProps: ProviderProps,
) => Store

export const buildProvider = <Store, Props = EmptyProps>(
  Context: Context<StoreApi<Store> | null>,
  storeBuilder: StoreBuilder<Store, Props>,
) => {
  const Provider = ({ children, ...props }: PropsWithChildren<Props>) => {
    const [store] = useState(() => createStore<Store>()(set => storeBuilder(set, props as Props)))

    return <Context.Provider value={store}>{children}</Context.Provider>
  }

  return Provider
}
