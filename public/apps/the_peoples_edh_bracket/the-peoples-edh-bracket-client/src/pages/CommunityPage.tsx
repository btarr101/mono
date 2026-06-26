import { Autocomplete, Box, Button, Group, Select, Skeleton, Stack, Table } from '@mantine/core'
import { MagnifyingGlassIcon } from '@phosphor-icons/react'
import { useWindowVirtualizer } from '@tanstack/react-virtual'
import { parseAsStringLiteral, useQueryState } from 'nuqs'
import { useLayoutEffect } from 'react'
import { Link } from 'react-router'

import { EmptyPlaceholder } from '../components/EmptyPlaceholder'
import { ViewablePersonProfileLine } from '../components/ViewablePersonProfileLine'
import { useReactVirtualScrollRestoration } from '../hooks/react-virtual-ext'
import { useDebouncedSearchPersons, useGetPersons } from '../hooks/usePersons'
import type { GetPersonsParamsSort } from '../types/bindings/GetPersonsParamsSort'

const PAGE_SIZE = 50

export const CommunityPage = () => {
  'use no memo'

  const [q, setQ] = useQueryState('q')
  const [sort, setSort] = useQueryState(
    'sort',
    parseAsStringLiteral<GetPersonsParamsSort>(['likes', 'followers', 'cards_rated']),
  )

  const [usedSearchPersons, { debouncedQ, isDebouncing }] = useDebouncedSearchPersons(q || null)
  const isAutocompleteLoading = isDebouncing || usedSearchPersons.isFetching

  const usedGetPersons = useGetPersons({
    q: debouncedQ,
    sort,
    page_size: PAGE_SIZE,
  })
  const persons = usedGetPersons.data?.pages.flat() ?? []

  const showEmptyMessage = !usedGetPersons.isLoading && persons.length === 0
  const showEndMessage =
    !usedGetPersons.hasNextPage &&
    (usedGetPersons.data?.pages.filter(page => page.length > 0).length ?? 0) > 1

  const virtualizer = useWindowVirtualizer({
    count: persons.length,
    estimateSize: () => 53,
    overscan: PAGE_SIZE,
  })

  const virtualItems = virtualizer.getVirtualItems()
  const first = virtualItems.at(0)?.start
  const end = virtualizer.getTotalSize() - (virtualItems.at(-1)?.end ?? 0)

  useReactVirtualScrollRestoration(virtualizer)

  // Infinite scrolling
  useLayoutEffect(() => {
    if (end === 0 && usedGetPersons.hasNextPage && !usedGetPersons.isFetching) {
      usedGetPersons.fetchNextPage()
    }
  }, [usedGetPersons, end])

  return (
    <Stack align="stretch" mih="100dvh" p="xl" w="100%">
      <Group w={'100%'}>
        <Autocomplete
          data={
            isAutocompleteLoading
              ? [{ value: '...', disabled: true }]
              : (usedSearchPersons.data?.pages.flat().map(({ username }) => username) ?? [])
          }
          filter={({ options }) => options}
          loading={usedGetPersons.isFetching}
          placeholder="Search for a person..."
          rightSection={<MagnifyingGlassIcon />}
          style={{ flex: 1 }}
          value={q ?? ''}
          onChange={newValue => setQ(newValue ?? undefined)}
        />
        <Select
          data={[
            { value: 'followers', label: '👥 Followers' },
            { value: 'cards_rated', label: '📝 Cards Rated' },
            { value: 'likes', label: '👍 Likes' },
          ]}
          placeholder="sort by"
          value={sort}
          onChange={newSort => setSort(newSort)}
        />
      </Group>
      <Table stickyHeader>
        <colgroup>
          <col style={{ width: '50%' }} />
          <col style={{ width: '11%' }} />
          <col style={{ width: '11%' }} />
          <col style={{ width: '11%' }} />
          <col style={{ width: '11%' }} />
          <col style={{ width: '6%' }} />
        </colgroup>
        <Table.Thead>
          <Table.Tr>
            <Table.Th>Person</Table.Th>
            <Table.Th>👥 Followers</Table.Th>
            <Table.Th>📝 Cards Rated</Table.Th>
            <Table.Th>👍 Likes</Table.Th>
            <Table.Th>👎 Dislikes</Table.Th>
            <Table.Th />
          </Table.Tr>
        </Table.Thead>
        <Table.Tbody>
          {usedGetPersons.isLoading ? (
            Array.from({ length: PAGE_SIZE }).map((_, index) => (
              <Table.Tr key={index}>
                <Table.Td colSpan={6}>
                  <Skeleton h={38} />
                </Table.Td>
              </Table.Tr>
            ))
          ) : (
            <>
              <Table.Tr h={first ?? 0} />
              {virtualizer.getVirtualItems().map(item => {
                const person = persons[item.index]!

                return (
                  <Table.Tr key={person.uuid}>
                    <Table.Td>
                      <Box w="fit-content">
                        <ViewablePersonProfileLine loading={false} person={person} />
                      </Box>
                    </Table.Td>
                    <Table.Td>{person.followers}</Table.Td>
                    <Table.Td>{person.cards_rated}</Table.Td>
                    <Table.Td>{person.likes}</Table.Td>
                    <Table.Td>{person.dislikes}</Table.Td>
                    <Table.Td ta="right">
                      <Button component={Link} to={{ pathname: `/community/${person.uuid}` }}>
                        View
                      </Button>
                    </Table.Td>
                  </Table.Tr>
                )
              })}
              <Table.Tr h={end ?? 0} />
            </>
          )}
        </Table.Tbody>
      </Table>
      {showEmptyMessage && (
        <EmptyPlaceholder subText="Try refining your search." title="🤔 No people found" />
      )}
      {showEndMessage && (
        <EmptyPlaceholder subText="No more people found." title="👋 That's all folks!" />
      )}
    </Stack>
  )
}
