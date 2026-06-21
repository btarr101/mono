import { Autocomplete, Box, Button, Group, Stack, Table } from '@mantine/core'
import { MagnifyingGlassIcon } from '@phosphor-icons/react'
import { useWindowVirtualizer } from '@tanstack/react-virtual'
import { useQueryState } from 'nuqs'
import { useEffect, useLayoutEffect, useRef, useState } from 'react'
import { Link } from 'react-router'

import { EmptyPlaceholder } from '../components/EmptyPlaceholder'
import { ViewablePersonProfileLine } from '../components/ViewablePersonProfileLine'
import { useDebouncedSearchPersons, useGetPersons } from '../hooks/usePersons'

const PAGE_SIZE = 50
const ROW_HEIGHT = 53

export const CommunityPage = () => {
  const [q, setQ] = useQueryState('q')

  const [usedSearchPersons, { debouncedQ, isDebouncing }] = useDebouncedSearchPersons(q || null)
  const usedGetPersons = useGetPersons({
    q: debouncedQ,
    page_size: PAGE_SIZE,
  })

  const isAutocompleteLoading = isDebouncing || usedSearchPersons.isFetching

  const persons = usedGetPersons.data?.pages.flat() ?? []

  const tableRef = useRef<HTMLTableElement>(null)
  const [scrollMargin, setScrollMargin] = useState(0)
  useLayoutEffect(() => {
    setScrollMargin(tableRef.current?.offsetTop ?? 0)
  }, [])

  const virtualizer = useWindowVirtualizer({
    count: persons.length,
    estimateSize: () => ROW_HEIGHT,
    overscan: 100,
    scrollMargin,
  })

  const virtualItems = virtualizer.getVirtualItems()
  const firstItem = virtualItems[0]
  const lastItem = virtualItems[virtualItems.length - 1]
  const paddingTop = firstItem ? firstItem.start - virtualizer.options.scrollMargin : 0
  const paddingBottom = lastItem
    ? virtualizer.getTotalSize() - (lastItem.end - virtualizer.options.scrollMargin)
    : 0

  const { hasNextPage, isFetchingNextPage, fetchNextPage } = usedGetPersons
  useEffect(() => {
    if (!lastItem) return
    if (lastItem.index >= persons.length - 1 && hasNextPage && !isFetchingNextPage) {
      fetchNextPage()
    }
  }, [lastItem, persons.length, hasNextPage, isFetchingNextPage, fetchNextPage])

  const showEndMessage =
    !usedGetPersons.hasNextPage &&
    (usedGetPersons.data?.pages.filter(page => page.length > 0).length ?? 0) > 1

  return (
    <Box p="xl" w="100%">
      <Stack gap="sm">
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
        </Group>
        <Table stickyHeader layout="fixed" ref={tableRef}>
          <Table.Thead>
            <Table.Tr>
              <Table.Th>Person</Table.Th>
              <Table.Th w={120}>Followers</Table.Th>
              <Table.Th w={120}>Cards Rated</Table.Th>
              <Table.Th w={120}>Likes</Table.Th>
              <Table.Th w={120}>Dislikes</Table.Th>
              <Table.Th ta="right" w={120} />
            </Table.Tr>
          </Table.Thead>
          <Table.Tbody>
            {paddingTop > 0 && (
              <Table.Tr>
                <Table.Td colSpan={6} h={paddingTop} p={0} />
              </Table.Tr>
            )}
            {virtualItems.length ? (
              virtualItems.map(virtualRow => {
                const person = persons[virtualRow.index]
                if (!person) return null

                return (
                  <Table.Tr h={ROW_HEIGHT} key={person.uuid}>
                    <Table.Td>
                      <Box w="fit-content">
                        <ViewablePersonProfileLine loading={false} person={person} />
                      </Box>
                    </Table.Td>
                    <Table.Td>2</Table.Td>
                    <Table.Td>45</Table.Td>
                    <Table.Td>2</Table.Td>
                    <Table.Td>0</Table.Td>
                    <Table.Td ta="right">
                      <Button component={Link} to={{ pathname: `/community/${person.uuid}` }}>
                        View
                      </Button>
                    </Table.Td>
                  </Table.Tr>
                )
              })
            ) : (
              <Table.Tr>
                <Table.Td colSpan={6} h={paddingBottom} px={0}>
                  <EmptyPlaceholder
                    subText="Try refining your search."
                    title="🤔 No persons found"
                  />
                </Table.Td>
              </Table.Tr>
            )}
            {showEndMessage && (
              <Table.Tr>
                <Table.Td colSpan={6} h={paddingBottom} px={0}>
                  <EmptyPlaceholder
                    subText="The journey is complete, you may rest now 🛌."
                    title="The end."
                  />
                </Table.Td>
              </Table.Tr>
            )}
            {paddingBottom > 0 && (
              <Table.Tr>
                <Table.Td colSpan={6} h={paddingBottom} p={0} />
              </Table.Tr>
            )}
          </Table.Tbody>
        </Table>
      </Stack>
    </Box>
  )
}
