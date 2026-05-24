import { type Context, type PropsWithChildren, useState } from 'react'
import { createStore, type StateCreator, type StoreApi } from 'zustand'

type StoreSet<T> = Parameters<StateCreator<T>>[0]
type StoreGet<T> = Parameters<StateCreator<T>>[1]
export type EmptyProps = object
export type StoreBuilder<Store, ProviderProps = EmptyProps> = (
  set: StoreSet<Store>,
  get: StoreGet<Store>,
  providerProps: ProviderProps,
) => Store

export const buildProvider = <Store, Props = EmptyProps>(
  Context: Context<StoreApi<Store> | null>,
  storeBuilder: StoreBuilder<Store, Props>,
) => {
  const Provider = ({ children, ...props }: PropsWithChildren<Props>) => {
    const [store] = useState(() =>
      createStore<Store>()((set, get) => storeBuilder(set, get, props as Props)),
    )

    return <Context.Provider value={store}>{children}</Context.Provider>
  }

  return Provider
}
