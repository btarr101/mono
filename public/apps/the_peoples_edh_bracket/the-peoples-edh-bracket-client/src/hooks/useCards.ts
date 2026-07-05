import { useDebouncedValue } from '@tanstack/react-pacer'
import { keepPreviousData, useInfiniteQuery, useQuery } from '@tanstack/react-query'

import { getCard, getCardMetrics, getCards } from '../api/cards'
import type { GetCardsParams } from '../types/bindings/GetCardsParams'

export const useGetCards = ({ q, sort, page_size }: Omit<GetCardsParams, 'page'>) =>
  useInfiniteQuery({
    queryKey: ['cards', q, sort, page_size],
    queryFn: ({ pageParam: page }) =>
      getCards({
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

export const useGetCard = (oracleId: string | null) =>
  useQuery({
    enabled: oracleId !== null,
    queryKey: ['card', oracleId],
    queryFn: () => (oracleId ? getCard(oracleId) : null),
  })

export const useGetCardMetrics = (oracleId: string) =>
  useQuery({
    queryKey: ['card', oracleId, 'metrics'],
    queryFn: () => getCardMetrics(oracleId),
  })

export const useSearchCards = (q: string | null) => useGetCards({ q, sort: null, page_size: 50 })

export const useDebouncedSearchCards = (q: string | null) => {
  const [debouncedQ, debouncer] = useDebouncedValue(q, { wait: 300 }, ({ isPending }) => isPending)
  const usedSearchCards = useSearchCards(debouncedQ)

  return [
    usedSearchCards,
    {
      debouncedQ,
      isDebouncing: debouncer.state,
    },
  ] as const
}
