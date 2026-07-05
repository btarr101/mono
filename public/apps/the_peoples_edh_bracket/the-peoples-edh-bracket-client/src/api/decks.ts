import type { GetTrackedDecksParams } from '../types/bindings/GetTrackedDecksParams'
import type { PostAnalyzeBody } from '../types/bindings/PostAnalyzeBody'
import type { PostAnalyzeDeckResponse } from '../types/bindings/PostAnalyzeDeckResponse'
import type { PostTrackedDeckBody } from '../types/bindings/PostTrackedDeckBody'
import type { TrackedDeck } from '../types/bindings/TrackedDeck'
import type { TrackedDeckWithAnalysis } from '../types/bindings/TrackedDeckWithAnalysis'
import type { TrackedDeckWithTotalPoints } from '../types/bindings/TrackedDeckWithTotalPoints'
import { api } from '.'

export const postAnalyze = async (body: PostAnalyzeBody) => {
  return api
    .post('decks/analyze', {
      json: body,
    })
    .json<PostAnalyzeDeckResponse>()
}

export const getTrackedDecks = async (params: GetTrackedDecksParams) => {
  const searchParams = new URLSearchParams()
  Object.entries(params).forEach(
    ([key, value]) => value !== null && searchParams.append(key, String(value)),
  )

  return api.get('decks', { searchParams }).json<TrackedDeckWithTotalPoints[]>()
}

export const postTrackedDeck = async (body: PostTrackedDeckBody) => {
  return api
    .post('decks', {
      json: body,
    })
    .json<TrackedDeck>()
}

export const getTrackedDeck = async (uuid: string) => {
  return api.get(`decks/${uuid}`).json<TrackedDeckWithAnalysis>()
}

export const deleteTrackedDeck = async (uuid: string) => {
  return api.delete(`decks/${uuid}`).json<TrackedDeck>()
}
