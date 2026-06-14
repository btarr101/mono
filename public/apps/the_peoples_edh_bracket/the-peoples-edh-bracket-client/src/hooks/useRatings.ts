import type { InfiniteData } from '@tanstack/react-query'
import {
  keepPreviousData,
  useInfiniteQuery,
  useMutation,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'

import {
  getRating,
  getRatingHistogramForCard,
  getRatings,
  putRating,
  putRatingReview,
} from '../api/ratings'
import type { CardRatingWithReviewsAndGlobalPoints } from '../types/bindings/CardRatingWithReviewsAndGlobalPoints'
import type { GetRatingHistogramParams } from '../types/bindings/GetRatingHistogramParams'
import type { GetRatingsParams } from '../types/bindings/GetRatingsParams'
import type { PutRatingReviewBody } from '../types/bindings/PutRatingReviewBody'

export const useRatings = ({
  card_oracle_id,
  rater_person_uuid,
  sort,
  page_size,
}: Omit<GetRatingsParams, 'page'>) =>
  useInfiniteQuery({
    queryKey: ['ratings', 'list', card_oracle_id, sort, page_size],
    queryFn: ({ pageParam: page }) =>
      getRatings({ card_oracle_id, rater_person_uuid, sort, page, page_size }),
    initialPageParam: 1,
    getNextPageParam: (lastPage, pages) => {
      if (lastPage.length === 0) return undefined
      return pages.length + 1
    },
    placeholderData: keepPreviousData,
  })

export const usePostRating = () => {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: putRating,
    onSuccess: () =>
      Promise.all([
        queryClient.invalidateQueries({
          queryKey: ['ratings'],
        }),
        queryClient.invalidateQueries({
          queryKey: ['rating'],
        }),
      ]),
  })
}

export const useRating = (uuid: string | null) =>
  useQuery({
    enabled: uuid !== null,
    queryKey: ['rating', uuid],
    queryFn: async () => (uuid ? await getRating(uuid) : null),
  })

export const usePersonRating = (oracleId: string, personUUID: string | null) =>
  useQuery({
    queryKey: ['ratings', oracleId, 'person', personUUID],
    queryFn: async () => {
      const ratings = await getRatings({
        card_oracle_id: oracleId,
        rater_person_uuid: personUUID,
        sort: null,
        page: 1,
        page_size: 1,
      })

      return ratings[0] ?? null
    },
    enabled: personUUID !== null,
  })

export const usePutRating = () => {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: putRating,
    onSuccess: ({ card_oracle_id }) =>
      Promise.all([
        queryClient.invalidateQueries({
          queryKey: ['ratings'],
        }),
        queryClient.invalidateQueries({
          queryKey: ['rating'],
        }),
        queryClient.invalidateQueries({
          queryKey: ['card', card_oracle_id],
        }),
      ]),
  })
}

export const usePutRatingReview = () => {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: ({ uuid, ...body }: { uuid: string } & PutRatingReviewBody) =>
      putRatingReview(uuid, body),
    onMutate: ({ uuid, like }) => {
      // Optimistic rendering - also incorrectly preserves order
      // as to not mess with UI
      queryClient.setQueriesData<InfiniteData<CardRatingWithReviewsAndGlobalPoints[]>>(
        { queryKey: ['ratings', 'list'] },
        data => {
          if (!data) return data

          return {
            ...data,
            pages: data.pages.map(page =>
              page.map(rating => {
                if (rating.uuid !== uuid) return rating

                const previousPersonReview = rating.reviews.person_review
                const nextPersonReview = previousPersonReview === like ? null : like

                let nextLikes = rating.reviews.likes
                let nextDislikes = rating.reviews.dislikes
                if (previousPersonReview === true) nextLikes -= 1
                if (previousPersonReview === false) nextDislikes -= 1
                if (nextPersonReview === true) nextLikes += 1
                if (nextPersonReview === false) nextDislikes += 1

                return {
                  ...rating,
                  reviews: {
                    ...rating.reviews,
                    person_review: nextPersonReview,
                    likes: nextLikes,
                    dislikes: nextDislikes,
                  },
                }
              }),
            ),
          }
        },
      )
    },
    onSuccess: () => Promise.all([queryClient.invalidateQueries({ queryKey: ['rating'] })]),
  })
}

export const useGetRatingHistogramForCard = (oracleId: string, params: GetRatingHistogramParams) =>
  useQuery({
    queryKey: ['ratings', 'histogram', 'card', oracleId, params],
    queryFn: async () => getRatingHistogramForCard(oracleId, params),
  })
