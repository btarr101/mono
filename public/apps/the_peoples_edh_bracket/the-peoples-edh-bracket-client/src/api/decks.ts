import { api, API_BASE_URL } from '.'
import type { PostAnalyzeBody } from '../types/bindings/PostAnalyzeBody'
import type { PostAnalyzeDeckResponse } from '../types/bindings/PostAnalyzeDeckResponse'

export const postAnalyze = async (body: PostAnalyzeBody) => {
  const uri = new URL(`${API_BASE_URL}/decks/analyze`)

  return api
    .post(uri, {
      json: body,
    })
    .json<PostAnalyzeDeckResponse>()
}
