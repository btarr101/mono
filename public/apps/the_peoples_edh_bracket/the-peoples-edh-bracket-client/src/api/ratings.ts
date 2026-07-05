import type { CardRating } from '../types/bindings/CardRating'
import type { CardRatingEnriched } from '../types/bindings/CardRatingEnriched'
import type { GetRatingHistogramParams } from '../types/bindings/GetRatingHistogramParams'
import type { GetRatingsParams } from '../types/bindings/GetRatingsParams'
import type { PointsHistogramBucket } from '../types/bindings/PointsHistogramBucket'
import type { PutRatingBody } from '../types/bindings/PutRatingBody'
import type { PutRatingReviewBody } from '../types/bindings/PutRatingReviewBody'
import { api } from '.'

export const getRatings = async (params: GetRatingsParams) => {
  const searchParams = new URLSearchParams()
  Object.entries(params).forEach(
    ([key, value]) => value !== null && searchParams.append(key, String(value)),
  )

  return api.get('ratings', { searchParams }).json<CardRatingEnriched[]>()
}

export const putRating = async (body: PutRatingBody) => {
  return api
    .put('ratings', {
      json: body,
    })
    .json<CardRating>()
}

export const getRating = async (uuid: string) => {
  return api.get(`ratings/${uuid}`).json<CardRatingEnriched>()
}

export const putRatingReview = async (uuid: string, body: PutRatingReviewBody) => {
  return api.put(`ratings/${uuid}/review`, {
    json: body,
  })
}

export const getRatingHistogramForCard = async (
  oracleId: string,
  params: GetRatingHistogramParams,
) => {
  const searchParams = new URLSearchParams()
  Object.entries(params).forEach(
    ([key, value]) => value !== null && searchParams.append(key, String(value)),
  )

  return api
    .get(`ratings/histogram/card/${oracleId}`, { searchParams })
    .json<PointsHistogramBucket[]>()
}
