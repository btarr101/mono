import type { CardRating } from '../types/bindings/CardRating'
import type { CardRatingWithReviews } from '../types/bindings/CardRatingWithReviews'
import type { GetRatingsParams } from '../types/bindings/GetRatingsParams'
import type { PatchRatingBody } from '../types/bindings/PatchRatingBody'
import type { PostRatingBody } from '../types/bindings/PostRatingBody'
import type { PostReviewRatingBody } from '../types/bindings/PostReviewRatingBody'
import { api, API_BASE_URL } from '.'

export const getRatings = async (params: GetRatingsParams) => {
  const uri = new URL(`${API_BASE_URL}/ratings`)
  Object.entries(params).forEach(
    ([key, value]) => value !== null && uri.searchParams.append(key, String(value)),
  )

  return api.get(uri).json<CardRatingWithReviews[]>()
}

export const postRating = async (body: PostRatingBody) => {
  const uri = new URL(`${API_BASE_URL}/ratings`)

  return api
    .post(uri, {
      json: body,
    })
    .json<CardRating>()
}

export const getRating = async (uuid: string) => {
  const uri = new URL(`${API_BASE_URL}/ratings/${uuid}`)

  return api.get(uri).json<CardRatingWithReviews>()
}

export const patchRating = async (uuid: string, body: PatchRatingBody) => {
  const uri = new URL(`${API_BASE_URL}/ratings/${uuid}`)

  return api
    .patch(uri, {
      json: body,
    })
    .json<CardRating>()
}

export const postReviewRating = async (uuid: string, body: PostReviewRatingBody) => {
  const uri = new URL(`${API_BASE_URL}/ratings/${uuid}/review`)

  return api.post(uri, {
    json: body,
  })
}
