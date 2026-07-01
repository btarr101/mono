import { type ClientConfig } from '../types/bindings/ClientConfig'
import { api, API_BASE_URL } from '.'

export const getConfig = async () => {
  const uri = new URL(`${API_BASE_URL}/config`)

  return api.get(uri).json<ClientConfig>()
}
