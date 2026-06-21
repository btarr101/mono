import { Autocomplete, Box, Button, Group, Stack, Table } from '@mantine/core'
import { MagnifyingGlassIcon } from '@phosphor-icons/react'
import { useQueryState } from 'nuqs'
import { Link } from 'react-router'

import { ViewablePersonProfileLine } from '../components/ViewablePersonProfileLine'
import { useDebouncedSearchPersons, useGetPersons } from '../hooks/usePersons'

const PAGE_SIZE = 50

export const CommunityPage = () => {
  const [q, setQ] = useQueryState('q')

  const [usedSearchPersons, { debouncedQ, isDebouncing }] = useDebouncedSearchPersons(q || null)
  const usedGetPersons = useGetPersons({
    q: debouncedQ,
    page_size: PAGE_SIZE,
  })

  const isAutocompleteLoading = isDebouncing || usedSearchPersons.isFetching

  return (
    <Stack mih="100dvh" p="xl" w="100%">
      <Group w={'100%'}>
        <Autocomplete
          data={
            isAutocompleteLoading
              ? [{ value: '...', disabled: true }]
              : (usedSearchPersons.data?.pages.flat().map(({ username }) => username) ?? [])
          }
          filter={({ options }) => options}
          loading={usedGetPersons.isFetching}
          placeholder="Search for a card..."
          rightSection={<MagnifyingGlassIcon />}
          style={{ flex: 1 }}
          value={q ?? ''}
          onChange={newValue => setQ(newValue ?? undefined)}
        />
      </Group>
      <Table stickyHeader>
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
          {usedGetPersons.data?.pages.flat().map(person => (
            <Table.Tr key={person.uuid}>
              <Table.Td>
                <Box w="fit-content">
                  <ViewablePersonProfileLine loading={false} person={person} />
                </Box>
              </Table.Td>
              <Table.Td>2</Table.Td>
              <Table.Td>45</Table.Td>
              <Table.Td>2</Table.Td>
              <Table.Td>0</Table.Td>
              <Table.Td>
                <Button component={Link} to={{ pathname: `/community/${person.uuid}` }}>
                  View
                </Button>
              </Table.Td>
            </Table.Tr>
          ))}
        </Table.Tbody>
      </Table>
    </Stack>
  )
}
