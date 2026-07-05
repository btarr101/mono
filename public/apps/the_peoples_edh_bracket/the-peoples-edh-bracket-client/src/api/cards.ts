import type { CardMetrics } from '../types/bindings/CardMetrics'
import type { CardWithMetrics } from '../types/bindings/CardWithMetrics'
import type { GetCardsParams } from '../types/bindings/GetCardsParams'
import { api } from '.'

export const getCards = async (params: GetCardsParams) => {
  const searchParams = new URLSearchParams()
  Object.entries(params).forEach(
    ([key, value]) => value !== null && searchParams.append(key, String(value)),
  )

  return api.get('cards', { searchParams }).json<CardWithMetrics[]>()
}

export const getCard = async (oracleId: string) => {
  return api.get(`cards/${oracleId}`).json<CardWithMetrics>()
}

export const getCardMetrics = async (oracleId: string) => {
  return api.get(`cards/${oracleId}/pts`).json<CardMetrics>()
}
