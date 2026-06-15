import { api, API_BASE_URL } from '.'
import type { AnalyzedDeck } from '../types/bindings/AnalyzedDeck'
import type { PostAnalyzeBody } from '../types/bindings/PostAnalyzeBody'

export const postAnalyze = async (body: PostAnalyzeBody) => {
  const uri = new URL(`${API_BASE_URL}/decks/analyze`)

  return api
    .post(uri, {
      json: body,
    })
    .json<AnalyzedDeck>()
}
