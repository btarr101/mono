import { Autocomplete, Button, Group, Select, Skeleton, Stack, Table, Text } from '@mantine/core'
import { FilesIcon, MagnifyingGlassIcon } from '@phosphor-icons/react'
import { GlobeIcon } from '@phosphor-icons/react/dist/ssr'
import { useWindowVirtualizer } from '@tanstack/react-virtual'
import { uniq } from 'lodash-es'
import { parseAsStringLiteral, useQueryState } from 'nuqs'
import { useLayoutEffect } from 'react'
import { Link } from 'react-router'

import { EmptyPlaceholder } from '../../components/EmptyPlaceholder'
import { PointsNumberFormatter } from '../../components/PointsNumberFormatter'
import { useReactVirtualScrollRestoration } from '../../hooks/react-virtual-ext'
import { useDebouncedSearchTrackedDecks, useGetTrackedDecks } from '../../hooks/useTrackedDecks'
import type { GetTrackedDecksParamsSort } from '../../types/bindings/GetTrackedDecksParamsSort'
import { formatTimeStamp } from '../../util'

const PAGE_SIZE = 50

export type TrackedDecksPanelContentProps = {
  personUUID: string
}

export const TrackedDecksPanelContent = ({ personUUID }: TrackedDecksPanelContentProps) => {
  'use no memo'

  const [q, setQ] = useQueryState('t-q')
  const [sort, setSort] = useQueryState(
    't-sort',
    parseAsStringLiteral([
      'recent',
      'most_points',
      'least_points',
      'most_personal_points',
      'least_personal_points',
    ] as const satisfies GetTrackedDecksParamsSort[]).withDefault('recent'),
  )

  const [usedSearchTrackedDecks, { debouncedQ, isDebouncing }] = useDebouncedSearchTrackedDecks({
    q: q || null,
    trackerPersonUUID: personUUID,
  })
  const isAutocompleteLoading = isDebouncing || usedSearchTrackedDecks.isFetching

  const usedTrackedDecks = useGetTrackedDecks({
    q: debouncedQ,
    tracker_person_uuid: personUUID,
    sort,
    page_size: PAGE_SIZE,
  })
  const trackedDecks = usedTrackedDecks.data?.pages.flat() ?? []

  const showEmptyMessage = !usedTrackedDecks.isLoading && trackedDecks.length === 0
  const showEndMessage =
    !usedTrackedDecks.hasNextPage &&
    (usedTrackedDecks.data?.pages.filter(page => page.length > 0).length ?? 0) > 1

  const virtualizer = useWindowVirtualizer({
    count: trackedDecks.length,
    estimateSize: () => 150,
    overscan: PAGE_SIZE,
  })

  const virtualItems = virtualizer.getVirtualItems()
  const first = virtualItems.at(0)?.start ?? 0
  const end = virtualizer.getTotalSize() - (virtualItems.at(-1)?.end ?? 0)

  useReactVirtualScrollRestoration(virtualizer)

  // Infinite scrolling
  useLayoutEffect(() => {
    if (end === 0 && usedTrackedDecks.hasNextPage && !usedTrackedDecks.isFetching) {
      usedTrackedDecks.fetchNextPage()
    }
  }, [usedTrackedDecks, end])

  return (
    <Stack p="lg" px={0}>
      <Group w={'100%'}>
        <Autocomplete
          data={
            isAutocompleteLoading
              ? [{ value: '...', disabled: true }]
              : uniq(usedSearchTrackedDecks.data?.pages.flat().map(({ name }) => name))
          }
          placeholder="Search for a tracked deck..."
          rightSection={<MagnifyingGlassIcon />}
          style={{ flex: 1 }}
          value={q ?? ''}
          onChange={newValue => setQ(newValue ?? undefined)}
        />
        <Select
          allowDeselect={false}
          data={[
            {
              value: 'recent',
              label: '⏲️ Most Recent',
            },
            {
              value: 'most_points',
              label: '🏆 Most Points',
            },
            {
              value: 'least_points',
              label: '⬇️ Least Points',
            },
            {
              value: 'most_personal_points',
              label: '👤🏆 Most Personal Points',
            },
            {
              value: 'least_personal_points',
              label: '👤⬇️ Least Personal Points',
            },
          ]}
          defaultValue="recent"
          disabled={trackedDecks.length === 0}
          value={sort}
          onChange={newSort => setSort(newSort)}
        />
      </Group>
      <Table stickyHeader>
        <colgroup>
          <col style={{ width: '30%' }} />
          <col style={{ width: '14%' }} />
          <col style={{ width: '14%' }} />
          <col style={{ width: '14%' }} />
          <col style={{ width: '14%' }} />
          <col style={{ width: '14%' }} />
        </colgroup>
        <Table.Thead>
          <Table.Tr>
            <Table.Th>Name</Table.Th>
            <Table.Th>pts</Table.Th>
            <Table.Th>ppts</Table.Th>
            <Table.Th>Source</Table.Th>
            <Table.Th>⏲️ Started Tracking</Table.Th>
            <Table.Th />
          </Table.Tr>
        </Table.Thead>
        <Table.Tbody>
          {usedTrackedDecks.isLoading ? (
            Array.from({ length: PAGE_SIZE }).map((_, index) => (
              <Table.Tr key={index}>
                <Table.Td colSpan={6}>
                  <Skeleton h={36} />
                </Table.Td>
              </Table.Tr>
            ))
          ) : (
            <>
              <Table.Tr h={first ?? 0} />
              {virtualizer.getVirtualItems().map(item => {
                const trackedDeck = trackedDecks[item.index]!

                return (
                  <Table.Tr key={trackedDeck.uuid}>
                    <Table.Td>{trackedDeck.name}</Table.Td>
                    <Table.Td>
                      <PointsNumberFormatter
                        points={trackedDeck.total_global_points}
                        suffix=" pts"
                      />
                    </Table.Td>
                    <Table.Td>
                      <PointsNumberFormatter
                        points={trackedDeck.total_personal_points}
                        suffix=" ppts"
                      />
                    </Table.Td>
                    <Table.Td>
                      <Group gap="xs" wrap="nowrap">
                        {trackedDeck.url_source ? (
                          <>
                            <GlobeIcon size={32} />
                            <Text textWrap="nowrap">URL</Text>
                          </>
                        ) : (
                          <>
                            <FilesIcon size={32} />
                            <Text textWrap="nowrap">Decklist</Text>
                          </>
                        )}
                      </Group>
                    </Table.Td>
                    <Table.Td>{formatTimeStamp(trackedDeck.created_at)}</Table.Td>
                    <Table.Td ta="right">
                      <Button component={Link} to={`/analyze/${trackedDeck.uuid}`}>
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
        <EmptyPlaceholder subText="Try refining your search." title="🤔 No tracked decks found" />
      )}
      {showEndMessage && <EmptyPlaceholder subText="Got em' all." title="You're dunzo" />}
    </Stack>
  )
}
