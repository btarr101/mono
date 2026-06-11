import type { HomeMetrics } from '../types/bindings/HomeMetrics'
import { api, API_BASE_URL } from '.'

export const getHomeMetrics = async () => {
  const uri = new URL(`${API_BASE_URL}/home/metrics`)

  return api.get(uri).json<HomeMetrics[]>()
}
