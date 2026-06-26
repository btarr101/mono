import type { GetTrackedDecksParams } from '../types/bindings/GetTrackedDecksParams'
import type { PostAnalyzeBody } from '../types/bindings/PostAnalyzeBody'
import type { PostAnalyzeDeckResponse } from '../types/bindings/PostAnalyzeDeckResponse'
import type { PostTrackedDeckBody } from '../types/bindings/PostTrackedDeckBody'
import type { TrackedDeck } from '../types/bindings/TrackedDeck'
import type { TrackedDeckWithAnalysis } from '../types/bindings/TrackedDeckWithAnalysis'
import type { TrackedDeckWithTotalPoints } from '../types/bindings/TrackedDeckWithTotalPoints'
import { api, API_BASE_URL } from '.'

export const postAnalyze = async (body: PostAnalyzeBody) => {
  const uri = new URL(`${API_BASE_URL}/decks/analyze`)

  return api
    .post(uri, {
      json: body,
    })
    .json<PostAnalyzeDeckResponse>()
}

export const getTrackedDecks = async (params: GetTrackedDecksParams) => {
  const uri = new URL(`${API_BASE_URL}/decks`)
  Object.entries(params).forEach(
    ([key, value]) => value !== null && uri.searchParams.append(key, String(value)),
  )

  return api.get(uri).json<TrackedDeckWithTotalPoints[]>()
}

export const postTrackedDeck = async (body: PostTrackedDeckBody) => {
  const uri = new URL(`${API_BASE_URL}/decks`)
  return api
    .post(uri, {
      json: body,
    })
    .json<TrackedDeck>()
}

export const getTrackedDeck = async (uuid: string) => {
  const uri = new URL(`${API_BASE_URL}/decks/${uuid}`)

  return api.get(uri).json<TrackedDeckWithAnalysis>()
}

export const deleteTrackedDeck = async (uuid: string) => {
  const uri = new URL(`${API_BASE_URL}/decks/${uuid}`)

  return api.delete(uri).json<TrackedDeck>()
}
