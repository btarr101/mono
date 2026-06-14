import { useInfiniteQuery, useMutation, useQuery, useQueryClient } from '@tanstack/react-query'

import { debugPostPerson, getPerson, getPersons } from '../api/persons'
import type { GetPersonsParams } from '../types/bindings/GetPersonsParams'
import { useAuthState } from './useAuth'

export const usePersons = ({ q, page_size }: Omit<GetPersonsParams, 'page'>) =>
  useInfiniteQuery({
    queryKey: ['persons', q, page_size],
    queryFn: ({ pageParam: page }) =>
      getPersons({
        q,
        page: page,
        page_size,
      }),
    initialPageParam: 1,
    getNextPageParam: (_, pages) => pages.length + 1,
  })

export const useSearchPersons = (q: string | null) => usePersons({ q, page_size: 10 })

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
  const uuid = authState.ty === 'debug' ? authState.personUUID : null

  return usePerson(uuid)
}
