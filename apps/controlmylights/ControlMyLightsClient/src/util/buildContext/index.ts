import { buildContextAndHooks } from './buildContextAndHook'
import { buildProvider, type EmptyProps, type StoreBuilder } from './buildProvider'

export const buildContext = <Store, ProviderProps = EmptyProps>(
  storeBuilder: StoreBuilder<Store, ProviderProps>,
) => {
  const { Context, useStoreApi } = buildContextAndHooks<Store>()
  const StoreProvider = buildProvider(Context, storeBuilder)

  return {
    StoreProvider,
    useStoreApi,
  }
}
