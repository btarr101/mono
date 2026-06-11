import type { CardRating } from '../types/bindings/CardRating'
import type { CardRatingWithReviewsAndGlobalPoints } from '../types/bindings/CardRatingWithReviewsAndGlobalPoints'
import type { GetRatingHistogramParams } from '../types/bindings/GetRatingHistogramParams'
import type { GetRatingsParams } from '../types/bindings/GetRatingsParams'
import type { PutRatingBody } from '../types/bindings/PutRatingBody'
import type { PutRatingReviewBody } from '../types/bindings/PutRatingReviewBody'
import type { RatingHistogramBucket } from '../types/bindings/RatingHistogramBucket'
import { api, API_BASE_URL } from '.'

export const getRatings = async (params: GetRatingsParams) => {
  const uri = new URL(`${API_BASE_URL}/ratings`)
  Object.entries(params).forEach(
    ([key, value]) => value !== null && uri.searchParams.append(key, String(value)),
  )

  return api.get(uri).json<CardRatingWithReviewsAndGlobalPoints[]>()
}

export const putRating = async (body: PutRatingBody) => {
  const uri = new URL(`${API_BASE_URL}/ratings`)

  return api
    .put(uri, {
      json: body,
    })
    .json<CardRating>()
}

export const getRating = async (uuid: string) => {
  const uri = new URL(`${API_BASE_URL}/ratings/${uuid}`)

  return api.get(uri).json<CardRatingWithReviewsAndGlobalPoints>()
}

export const putRatingReview = async (uuid: string, body: PutRatingReviewBody) => {
  const uri = new URL(`${API_BASE_URL}/ratings/${uuid}/review`)

  return api.put(uri, {
    json: body,
  })
}

export const getRatingHistogramForCard = async (
  oracleId: string,
  params: GetRatingHistogramParams,
) => {
  const uri = new URL(`${API_BASE_URL}/ratings/histogram/card/${oracleId}`)
  Object.entries(params).forEach(
    ([key, value]) => value !== null && uri.searchParams.append(key, String(value)),
  )

  return api.get(uri).json<RatingHistogramBucket[]>()
}
