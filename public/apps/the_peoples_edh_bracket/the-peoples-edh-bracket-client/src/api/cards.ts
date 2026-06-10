import type { CardWithGlobalPoints } from '../types/bindings/CardWithGlobalPoints'
import type { GetCardsParams } from '../types/bindings/GetCardsParams'
import { api, API_BASE_URL } from '.'

export const getCards = async (params: GetCardsParams) => {
  const uri = new URL(`${API_BASE_URL}/cards`)
  Object.entries(params).forEach(
    ([key, value]) => value !== null && uri.searchParams.append(key, String(value)),
  )

  return api.get(uri).json<CardWithGlobalPoints[]>()
}

export const getCard = async (oracleId: string) => {
  const uri = new URL(`${API_BASE_URL}/cards/${oracleId}`)

  return api.get(uri).json<CardWithGlobalPoints>()
}
