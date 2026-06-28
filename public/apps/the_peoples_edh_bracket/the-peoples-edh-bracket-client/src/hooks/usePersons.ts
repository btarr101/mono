import { useDebouncedValue } from '@tanstack/react-pacer'
import {
  keepPreviousData,
  useInfiniteQuery,
  useMutation,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'

import {
  debugPostPerson,
  getMe,
  getPerson,
  getPersons,
  postFollowPerson,
  postUnfollowPerson,
} from '../api/persons'
import type { GetPersonsParams } from '../types/bindings/GetPersonsParams'
import { useAuthState } from './useAuth'

export const useGetPersons = (params: Omit<GetPersonsParams, 'page'>) =>
  useInfiniteQuery({
    queryKey: ['persons', params],
    queryFn: ({ pageParam: page }) =>
      getPersons({
        ...params,
        page: page,
      }),
    initialPageParam: 1,
    getNextPageParam: (lastPage, pages) =>
      lastPage.length < params.page_size ? undefined : pages.length + 1,
    placeholderData: keepPreviousData,
  })

export type UseSearchPersonsOptions = {
  personFollowing?: string
  personFollowee?: string
}

export const useSearchPersons = (q: string | null, options?: UseSearchPersonsOptions) =>
  useGetPersons({
    person_following: options?.personFollowing ?? null,
    person_followee: options?.personFollowee ?? null,
    q,
    sort: null,
    page_size: 10,
  })

export const useDebouncedSearchPersons = (q: string | null, options?: UseSearchPersonsOptions) => {
  const [debouncedQ, debouncer] = useDebouncedValue(q, { wait: 200 }, ({ isPending }) => isPending)
  const usedSearchCards = useSearchPersons(debouncedQ, options)

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
    queryKey: ['persons', uuid],
    queryFn: async () => (uuid ? await getPerson(uuid) : null),
    staleTime: Infinity,
  })

export const useMe = () => {
  const [authState] = useAuthState()
  const useMe = useQuery({
    queryKey: ['persons', 'me', authState],
    queryFn: getMe,
    staleTime: Infinity,
    enabled: authState.ty !== null,
    placeholderData: keepPreviousData,
  })

  return useMe
}

export const useFollowPerson = () => {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: postFollowPerson,
    onSuccess: () =>
      queryClient.invalidateQueries({
        queryKey: ['persons'],
      }),
  })
}

export const useUnfollowPerson = () => {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: postUnfollowPerson,
    onSuccess: () =>
      queryClient.invalidateQueries({
        queryKey: ['persons'],
      }),
  })
}
