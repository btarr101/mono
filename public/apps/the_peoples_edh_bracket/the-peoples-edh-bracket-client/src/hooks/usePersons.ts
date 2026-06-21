import { useDebouncedValue } from '@tanstack/react-pacer'
import {
  keepPreviousData,
  useInfiniteQuery,
  useMutation,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'

import { debugPostPerson, getMe, getPerson, getPersons } from '../api/persons'
import type { GetPersonsParams } from '../types/bindings/GetPersonsParams'
import { useAuthState } from './useAuth'

export const useGetPersons = ({ q, page_size }: Omit<GetPersonsParams, 'page'>) =>
  useInfiniteQuery({
    queryKey: ['persons', q, page_size],
    queryFn: ({ pageParam: page }) =>
      getPersons({
        q,
        page: page,
        page_size,
      }),
    initialPageParam: 1,
    getNextPageParam: (lastPage, pages) =>
      lastPage.length < page_size ? undefined : pages.length + 1,
    placeholderData: keepPreviousData,
  })

export const useSearchPersons = (q: string | null) => useGetPersons({ q, page_size: 10 })

export const useDebouncedSearchPersons = (q: string | null) => {
  const [debouncedQ, debouncer] = useDebouncedValue(q, { wait: 200 }, ({ isPending }) => isPending)
  const usedSearchCards = useSearchPersons(debouncedQ)

  return [
    usedSearchCards,
    {
      debouncedQ,
      isDebouncing: debouncer.state,
    },
  ] as const
}

export const useDebugPostPerson = () => {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: debugPostPerson,
    onSuccess: () =>
      queryClient.invalidateQueries({
        queryKey: ['persons'],
      }),
  })
}

export const usePerson = (uuid: string | null) =>
  useQuery({
    queryKey: ['person', uuid],
    queryFn: async () => (uuid ? await getPerson(uuid) : null),
    staleTime: Infinity,
  })

export const useMe = () => {
  const [authState] = useAuthState()
  const useMe = useQuery({
    queryKey: ['person', 'me', authState],
    queryFn: getMe,
    staleTime: Infinity,
    enabled: authState.ty !== null,
  })

  return useMe
}
