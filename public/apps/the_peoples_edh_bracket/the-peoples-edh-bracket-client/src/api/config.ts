import { type ClientConfig } from '../types/bindings/ClientConfig'
import { api } from '.'

export const getConfig = async () => {
  return api.get('config').json<ClientConfig>()
}
