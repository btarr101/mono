import { buildContextAndHook } from './buildContextAndHook'
import { buildProvider, type EmptyProps, type StoreBuilder } from './buildProvider'

export const buildContext = <Store, ProviderProps = EmptyProps>(
  storeBuilder: StoreBuilder<Store, ProviderProps>,
) => {
  const { Context, useStoreContext } = buildContextAndHook<Store>()
  const StoreProvider = buildProvider(Context, storeBuilder)

  return {
    StoreProvider,
    useStoreContext,
  }
}
