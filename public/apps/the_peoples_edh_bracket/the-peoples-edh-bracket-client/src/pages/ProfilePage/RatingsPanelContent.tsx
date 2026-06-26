import { Autocomplete, Button, Group, Select, Skeleton, Stack, Table } from '@mantine/core'
import { MagnifyingGlassIcon } from '@phosphor-icons/react'
import { useWindowVirtualizer } from '@tanstack/react-virtual'
import { parseAsStringLiteral, useQueryState } from 'nuqs'
import { useLayoutEffect } from 'react'
import { Link } from 'react-router'

import { EmptyPlaceholder } from '../../components/EmptyPlaceholder'
import { PointsNumberFormatter } from '../../components/PointsNumberFormatter'
import { TableCard } from '../../components/TableCard'
import { useReactVirtualScrollRestoration } from '../../hooks/react-virtual-ext'
import { useGetCard } from '../../hooks/useCards'
import { useDebouncedSearchRatings, useGetRatings } from '../../hooks/useRatings'
import type { CardRatingEnriched } from '../../types/bindings/CardRatingEnriched'
import { formatTimeStamp } from '../../util'

const PAGE_SIZE = 50

export type RatingsPanelContentProps = {
  personUUID: string
}

export const RatingsPanelContent = ({ personUUID }: RatingsPanelContentProps) => {
  'use no memo'

  const [q, setQ] = useQueryState('ratings-q')
  const [sort, setSort] = useQueryState(
    'rating-sort',
    parseAsStringLiteral([
      'liked',
      'disliked',
      'controversial',
      'recent',
      'highest_rated',
      'lowest_rated',
    ] as const).withDefault('liked'),
  )

  const [usedSearchRatings, { debouncedQ, isDebouncing }] = useDebouncedSearchRatings({
    q: q || null,
    raterPersonUUID: personUUID,
  })
  const isAutocompleteLoading = isDebouncing || usedSearchRatings.isFetching

  const usedGetRatings = useGetRatings({
    q: debouncedQ,
    rater_person_uuid: personUUID,
    card_oracle_id: null,
    sort,
    page_size: PAGE_SIZE,
  })
  const ratings = usedGetRatings.data?.pages.flat() ?? []

  const showEmptyMessage = !usedGetRatings.isLoading && ratings.length === 0
  const showEndMessage =
    !usedGetRatings.hasNextPage &&
    (usedGetRatings.data?.pages.filter(page => page.length > 0).length ?? 0) > 1

  const virtualizer = useWindowVirtualizer({
    count: ratings.length,
    estimateSize: () => 150,
    overscan: PAGE_SIZE,
  })

  const virtualItems = virtualizer.getVirtualItems()
  const first = virtualItems.at(0)?.start ?? 0
  const end = virtualizer.getTotalSize() - (virtualItems.at(-1)?.end ?? 0)

  useReactVirtualScrollRestoration(virtualizer)

  // Infinite scrolling
  useLayoutEffect(() => {
    if (end === 0 && usedGetRatings.hasNextPage && !usedGetRatings.isFetching) {
      usedGetRatings.fetchNextPage()
    }
  }, [usedGetRatings, end])

  return (
    <Stack p="lg" px={0}>
      <Group w={'100%'}>
        <Autocomplete
          data={
            isAutocompleteLoading
              ? [{ value: '...', disabled: true }]
              : usedSearchRatings.data?.pages.flat().map(({ card_name }) => card_name)
          }
          placeholder="Search for a card..."
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
              value: 'liked',
              label: '👍 Most Liked',
            },
            {
              value: 'disliked',
              label: '👎 Most Disliked',
            },
            {
              value: 'controversial',
              label: '🔥 Most Controversial',
            },
            {
              value: 'highest_rated',
              label: '👑 Highest Rated',
            },
            {
              value: 'lowest_rated',
              label: '🗑️ Lowest Rated',
            },
          ]}
          defaultValue="recent"
          disabled={ratings.length === 0}
          value={sort}
          onChange={newSort => setSort(newSort)}
        />
      </Group>
      <Table stickyHeader>
        <colgroup>
          <col style={{ width: '28%' }} />
          <col style={{ width: '12%' }} />
          <col style={{ width: '12%' }} />
          <col style={{ width: '12%' }} />
          <col style={{ width: '12%' }} />
          <col style={{ width: '12%' }} />
          <col style={{ width: '12%' }} />
        </colgroup>
        <Table.Thead>
          <Table.Tr>
            <Table.Th>Card</Table.Th>
            <Table.Th>pts</Table.Th>
            <Table.Th>ppts</Table.Th>
            <Table.Th style={{ textWrap: 'nowrap' }}>👍 Likes</Table.Th>
            <Table.Th style={{ textWrap: 'nowrap' }}>👎 Dislikes</Table.Th>
            <Table.Th style={{ textWrap: 'nowrap' }}>⏲️ Rated</Table.Th>
            <Table.Th />
          </Table.Tr>
        </Table.Thead>
        <Table.Tbody>
          {usedGetRatings.isLoading ? (
            Array.from({ length: PAGE_SIZE }).map((_, index) => (
              <Table.Tr key={index}>
                <Table.Td colSpan={7}>
                  <Skeleton h={36} />
                </Table.Td>
              </Table.Tr>
            ))
          ) : (
            <>
              <Table.Tr h={first ?? 0} />
              {virtualizer.getVirtualItems().map(item => {
                const rating = ratings[item.index]!

                return <RatingRow key={rating.uuid} rating={rating} />
              })}
              <Table.Tr h={end ?? 0} />
            </>
          )}
        </Table.Tbody>
        {/*TODO GHOSTS PLUS PLACEHOLDERS*/}
      </Table>
      {showEmptyMessage && (
        <EmptyPlaceholder subText="Try refining your search." title="🤔 No people found" />
      )}
      {showEndMessage && (
        <EmptyPlaceholder subText="You've seen all the ratings." title="🎁 It's a wrap" />
      )}
    </Stack>
  )
}

type RatingRowProps = { rating: CardRatingEnriched }

const RatingRow = ({ rating }: RatingRowProps) => {
  const card = useGetCard(rating.card_oracle_id)

  return (
    <Table.Tr>
      <Table.Td>
        <Group wrap="nowrap">
          <TableCard imageUri={card.data?.image_uri} />
          {card.data?.name ?? '...'}
        </Group>
      </Table.Td>
      <Table.Td>
        <PointsNumberFormatter points={rating.global_points} suffix=" pts" />
      </Table.Td>
      <Table.Td>
        <PointsNumberFormatter points={rating.points} suffix=" ppts" />
      </Table.Td>
      <Table.Td>{rating.reviews.likes}</Table.Td>
      <Table.Td>{rating.reviews.dislikes}</Table.Td>
      <Table.Td>{formatTimeStamp(rating.created_at)}</Table.Td>
      <Table.Td ta="right">
        <Button
          component={Link}
          to={{ pathname: `/browse/${rating.card_oracle_id}`, search: `?pinned=${rating.uuid}` }}
        >
          View
        </Button>
      </Table.Td>
    </Table.Tr>
  )
}
