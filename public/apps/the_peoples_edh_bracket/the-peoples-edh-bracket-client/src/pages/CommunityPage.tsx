import { Autocomplete, Box, Button, Group, Stack, Table } from '@mantine/core'
import { MagnifyingGlassIcon } from '@phosphor-icons/react'
import { useWindowVirtualizer } from '@tanstack/react-virtual'
import { useQueryState } from 'nuqs'
import { useLayoutEffect, useRef, useState } from 'react'
import { Link } from 'react-router'

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

  const personsD = usedGetPersons.data?.pages.flat() ?? []
  const persons = Array.from({ length: 20 }).flatMap(() => personsD)

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
        <Table stickyHeader ref={tableRef}>
          <Table.Thead>
            <Table.Tr>
              <Table.Th>Person</Table.Th>
              <Table.Th>Followers</Table.Th>
              <Table.Th>Cards Rated</Table.Th>
              <Table.Th>Likes</Table.Th>
              <Table.Th>Dislikes</Table.Th>
              <Table.Th ta="right" />
            </Table.Tr>
          </Table.Thead>
          <Table.Tbody>
            {paddingTop > 0 && (
              <Table.Tr>
                <Table.Td colSpan={6} h={paddingTop} p={0} />
              </Table.Tr>
            )}
            {virtualItems.map(virtualRow => {
              const person = persons[virtualRow.index]
              if (!person) return null

              return (
                <Table.Tr h={ROW_HEIGHT} key={virtualRow.key}>
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
            })}
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
