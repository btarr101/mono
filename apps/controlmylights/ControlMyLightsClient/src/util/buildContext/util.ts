import { type StoreApi, useStore } from 'zustand'

export const buildUseStoreState =
  <Store>(useStoreApi: () => StoreApi<Store>) =>
  <Selected = Store>(selector?: (store: Store) => Selected) => {
    const store = useStoreApi()
    const resolvedSelector = (selector ?? (store => store)) as (store: Store) => Selected

    return useStore(store, resolvedSelector)
  }

export const buildUseStoreSubscribe =
  <Store>(useStoreApi: () => StoreApi<Store>) =>
  (listener: (state: Store, prevState: Store) => void) =>
    useStoreApi().subscribe(listener)
