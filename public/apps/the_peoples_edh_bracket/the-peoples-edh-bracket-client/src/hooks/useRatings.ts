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
  patchRating,
  postRating,
  postReviewRating,
} from '../api/ratings'
import type { GetRatingHistogramParams } from '../types/bindings/GetRatingHistogramParams'
import type { GetRatingsParams } from '../types/bindings/GetRatingsParams'
import type { PatchRatingBody } from '../types/bindings/PatchRatingBody'
import type { PostReviewRatingBody } from '../types/bindings/PostReviewRatingBody'
import { usePersonUUID } from './useAuth'

export const useRatings = ({
  card_oracle_id,
  rater_person_uuid,
  sort,
  page_size,
}: Omit<GetRatingsParams, 'page'>) =>
  useInfiniteQuery({
    queryKey: ['ratings', card_oracle_id, sort, page_size],
    queryFn: ({ pageParam: page }) =>
      getRatings({ card_oracle_id, rater_person_uuid, sort, page, page_size }),
    initialPageParam: 1,
    getNextPageParam: (_, pages) => pages.length + 1,
    placeholderData: keepPreviousData,
  })

export const usePostRating = () => {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: postRating,
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

export const useMyCardRating = (oracleId: string) => {
  const personUUID = usePersonUUID()
  return useQuery({
    queryKey: ['rating', 'me', oracleId],
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
}

export const usePatchRating = () => {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: ({ uuid, ...body }: { uuid: string } & PatchRatingBody) => patchRating(uuid, body),
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

export const usePostReviewRating = () => {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: ({ uuid, ...body }: { uuid: string } & PostReviewRatingBody) =>
      postReviewRating(uuid, body),
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

export const useGetRatingHistogramForCard = (oracleId: string, params: GetRatingHistogramParams) =>
  useQuery({
    queryKey: ['ratings', 'histogram', 'card', oracleId, params],
    queryFn: async () => getRatingHistogramForCard(oracleId, params),
  })
