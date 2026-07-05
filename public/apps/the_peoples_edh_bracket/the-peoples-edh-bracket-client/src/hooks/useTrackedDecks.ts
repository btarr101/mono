import { useDebouncedValue } from '@tanstack/react-pacer'
import { keepPreviousData, useInfiniteQuery } from '@tanstack/react-query'

import { getTrackedDecks } from '../api/decks'
import type { GetTrackedDecksParams } from '../types/bindings/GetTrackedDecksParams'

export const useGetTrackedDecks = ({
  tracker_person_uuid,
  q,
  sort,
  page_size,
}: Omit<GetTrackedDecksParams, 'page'>) =>
  useInfiniteQuery({
    queryKey: ['decks', { tracker_person_uuid, q, sort, page_size }],
    queryFn: async ({ pageParam: page }) =>
      getTrackedDecks({
        tracker_person_uuid,
        q,
        sort,
        page,
        page_size,
      }),
    initialPageParam: 1,
    getNextPageParam: (lastPage, pages) => {
      if (lastPage.length === 0) return undefined
      return pages.length + 1
    },
    placeholderData: keepPreviousData,
  })

export type useSearchTrackedDecksParams = {
  q: string | null
  trackerPersonUUID: string | null
}

export const useSearchTrackedDecks = ({ q, trackerPersonUUID }: useSearchTrackedDecksParams) =>
  useGetTrackedDecks({
    q,
    tracker_person_uuid: trackerPersonUUID,
    sort: null,
    page_size: 10,
  })

export const useDebouncedSearchTrackedDecks = ({
  q,
  trackerPersonUUID,
}: useSearchTrackedDecksParams) => {
  const [debouncedQ, debouncer] = useDebouncedValue(q, { wait: 200 }, ({ isPending }) => isPending)
  const usedSearchCards = useSearchTrackedDecks({ q: debouncedQ, trackerPersonUUID })

  return [
    usedSearchCards,
    {
      debouncedQ,
      isDebouncing: debouncer.state,
    },
  ] as const
}
