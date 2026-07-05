import type { HomeMetrics } from '../types/bindings/HomeMetrics'
import { api } from '.'

export const getHomeMetrics = async () => {
  const uri = 'home/metrics'

  return api.get(uri).json<HomeMetrics[]>()
}
