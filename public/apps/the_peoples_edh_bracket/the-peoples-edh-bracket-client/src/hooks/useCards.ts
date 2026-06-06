import {
  keepPreviousData,
  useInfiniteQuery,
  useQuery,
  useSuspenseInfiniteQuery,
} from '@tanstack/react-query'

import { getCard, getCards } from '../api/cards'
import type { GetCardsParams } from '../types/bindings/GetCardsParams'

export const useCards = ({ q, sort, page_size }: Omit<GetCardsParams, 'page'>) =>
  useInfiniteQuery({
    queryKey: ['cards', q, sort, page_size],
    queryFn: ({ pageParam: page }) =>
      getCards({
        q,
        sort,
        page: page,
        page_size,
      }),
    initialPageParam: 1,
    getNextPageParam: (_, pages) => pages.length + 1,
    placeholderData: keepPreviousData,
  })

export const useSuspenseCards = ({ q, sort, page_size }: Omit<GetCardsParams, 'page'>) =>
  useSuspenseInfiniteQuery({
    queryKey: ['cards', q, sort, page_size],
    queryFn: ({ pageParam: page }) =>
      getCards({
        q,
        sort,
        page: page,
        page_size,
      }),
    initialPageParam: 1,
    getNextPageParam: (_, pages) => pages.length + 1,
  })

export const useCard = (oracleId: string) =>
  useQuery({
    queryKey: ['card', oracleId],
    queryFn: () => getCard(oracleId),
  })

export const useSearchCards = (q: string | null) => useCards({ q, sort: null, page_size: 50 })
