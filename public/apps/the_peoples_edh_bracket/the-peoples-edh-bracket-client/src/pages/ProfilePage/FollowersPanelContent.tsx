import { Autocomplete, Box, Group, Select, Stack, Table } from '@mantine/core'
import { MagnifyingGlassIcon } from '@phosphor-icons/react'
import { useWindowVirtualizer } from '@tanstack/react-virtual'
import { parseAsStringLiteral, useQueryState } from 'nuqs'
import { useLayoutEffect } from 'react'

import { EmptyPlaceholder } from '../../components/EmptyPlaceholder'
import { ViewablePersonProfileLine } from '../../components/ViewablePersonProfileLine'
import { useReactVirtualScrollRestoration } from '../../hooks/react-virtual-ext'
import { useDebouncedSearchPersons, useGetPersons } from '../../hooks/usePersons'
import type { GetPersonsParamsSort } from '../../types/bindings/GetPersonsParamsSort'
import { formatTimeStamp } from '../../util'
import { TableRowLoader } from './TableRowLoader'

const PAGE_SIZE = 50

export type FollowersPanelContentProps = {
  personUUID: string
}

export const FollowersPanelContent = ({ personUUID }: FollowersPanelContentProps) => {
  'use no memo'

  const [q, setQ] = useQueryState('fwrs-q')
  const [sort, setSort] = useQueryState(
    'fwrs-sort',
    parseAsStringLiteral<GetPersonsParamsSort>(['likes', 'followers', 'cards_rated']),
  )

  const [usedSearchPersons, { debouncedQ, isDebouncing }] = useDebouncedSearchPersons(q || null, {
    personFollowing: personUUID,
  })
  const isAutocompleteLoading = isDebouncing || usedSearchPersons.isFetching

  const usedGetPersons = useGetPersons({
    person_following: personUUID,
    person_followee: null,
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
    <Stack p="lg" px={0}>
      <Group w={'100%'}>
        <Autocomplete
          data={
            isAutocompleteLoading
              ? [{ value: '...', disabled: true }]
              : (usedSearchPersons.data?.pages.flat().map(({ username }) => username) ?? [])
          }
          filter={({ options }) => options}
          loading={usedGetPersons.isFetching}
          miw="fit-content"
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
      <Table.ScrollContainer minWidth={'100%'}>
        <Table>
          <colgroup>
            <col style={{ width: '75%' }} />
            <col style={{ width: '25%' }} />
          </colgroup>
          <Table.Thead>
            <Table.Tr>
              <Table.Th>Follower</Table.Th>
              <Table.Th
                style={{
                  whiteSpace: 'nowrap',
                }}
              >
                Started Following
              </Table.Th>
            </Table.Tr>
          </Table.Thead>
          <Table.Tbody>
            {usedGetPersons.isLoading ? (
              <TableRowLoader colSpan={2} />
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
                      <Table.Td
                        style={{
                          whiteSpace: 'nowrap',
                        }}
                      >
                        {person.started_following && formatTimeStamp(person.started_following)}
                      </Table.Td>
                    </Table.Tr>
                  )
                })}
                <Table.Tr h={end ?? 0} />
              </>
            )}
          </Table.Tbody>
        </Table>
      </Table.ScrollContainer>
      {showEmptyMessage && <EmptyPlaceholder subText="..." title="No followers?" />}
      {showEndMessage && (
        <EmptyPlaceholder
          subText='Get it? "followed".'
          title="You *followed* through with this page."
        />
      )}
    </Stack>
  )
}
