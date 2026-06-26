import { Autocomplete, Box, Flex, Group, Select, Stack } from '@mantine/core'
import { MagnifyingGlassIcon } from '@phosphor-icons/react'
import { useWindowVirtualizer } from '@tanstack/react-virtual'
import { parseAsStringLiteral, useQueryState } from 'nuqs'
import { useLayoutEffect, useRef, useState } from 'react'

import { EmptyPlaceholder } from '../components/EmptyPlaceholder'
import { MtgCardButton, MtgCardButtonGhost } from '../components/MtgCardButton'
import { CARD_BUTTON_DIMENSIONS } from '../components/MtgCardButton.constants'
import { useReactVirtualScrollRestoration } from '../hooks/react-virtual-ext'
import { useDebouncedSearchCards, useGetCards } from '../hooks/useCards'
import type { GetCardsParamsSort } from '../types/bindings/GetCardsParamsSort'

const PAGE_SIZE = 50
const CARD_GAP = 16

export const BrowsePage = () => {
  const [q, setQ] = useQueryState('q')
  const [sort, setSort] = useQueryState(
    'sort',
    parseAsStringLiteral<GetCardsParamsSort>([
      'highest_rated',
      'lowest_rated',
      'most_controversial',
      'most_rated',
      'least_rated',
      'trending',
    ]),
  )

  const [usedSearchCards, { debouncedQ, isDebouncing }] = useDebouncedSearchCards(q || null)
  const isAutocompleteLoading = isDebouncing || usedSearchCards.isFetching

  const usedGetCards = useGetCards({
    q: debouncedQ,
    sort,
    page_size: PAGE_SIZE,
  })
  const cards = usedGetCards.data?.pages.flat() ?? []

  const showEmptyMessage = !usedGetCards.isLoading && cards.length === 0
  const showEndMessage =
    !usedGetCards.hasNextPage &&
    (usedGetCards.data?.pages.filter(page => page.length > 0).length ?? 0) > 1

  // Calculate size of row - since it's dynamic
  // --------------------------------------------------------------------
  const [containerWidth, setContainerWidth] = useState(0)
  const gridResizeObserverRef = useRef<ResizeObserver | null>(null)

  const gridContainerRef = (node: HTMLDivElement | null) => {
    gridResizeObserverRef.current?.disconnect()
    if (!node) return

    setContainerWidth(node.clientWidth)

    const resizeObserver = new ResizeObserver(entries => {
      const entry = entries[0]
      if (!entry) return
      setContainerWidth(entry.contentRect.width)
    })

    resizeObserver.observe(node)
    gridResizeObserverRef.current = resizeObserver
  }

  useLayoutEffect(
    () => () => {
      gridResizeObserverRef.current?.disconnect()
    },
    [],
  )

  const columnCount = Math.max(
    1,
    Math.floor(containerWidth / (CARD_BUTTON_DIMENSIONS.w + CARD_GAP)),
  )
  const rowCount = Math.ceil(cards.length / columnCount)
  const totalGridWidth = columnCount * (CARD_BUTTON_DIMENSIONS.w + CARD_GAP)
  // -----------------------------------------------------------------------

  const virtualizer = useWindowVirtualizer({
    count: rowCount,
    estimateSize: () => CARD_BUTTON_DIMENSIONS.h + CARD_GAP,
    overscan: Math.ceil(PAGE_SIZE / columnCount),
  })

  const virtualRows = virtualizer.getVirtualItems()
  const first = virtualRows.at(0)?.start ?? 0
  const end = virtualizer.getTotalSize() - (virtualRows.at(-1)?.end ?? 0)

  useReactVirtualScrollRestoration(virtualizer)

  // Infinite scrolling
  useLayoutEffect(() => {
    if (end === 0 && usedGetCards.hasNextPage && !usedGetCards.isFetching) {
      usedGetCards.fetchNextPage()
    }
  }, [usedGetCards, end])

  return (
    <Stack mih="100dvh" p="xl" w="100%">
      <Group w={'100%'}>
        <Autocomplete
          data={
            isAutocompleteLoading
              ? [{ value: '...', disabled: true }]
              : (usedSearchCards.data?.pages.flat().map(({ name }) => name) ?? [])
          }
          filter={({ options }) => options}
          loading={usedGetCards.isFetching}
          placeholder="Search for a card..."
          rightSection={<MagnifyingGlassIcon />}
          style={{ flex: 1 }}
          value={q ?? ''}
          onChange={newValue => setQ(newValue ?? undefined)}
        />
        <Select
          data={[
            { value: 'highest_rated', label: '👑 Highest Rated' },
            { value: 'lowest_rated', label: '🗑️ Lowest Rated' },
            { value: 'most_controversial', label: '⚔️ Most Controversial' },
            { value: 'most_rated', label: '👀 Most Rated' },
            { value: 'least_rated', label: '👻 Least Rated' },
            { value: 'trending', label: '🔥 Trending' },
          ]}
          placeholder="sort by"
          value={sort}
          onChange={newSort => setSort(newSort)}
        />
      </Group>
      <Stack align="center" gap={CARD_GAP} ref={gridContainerRef} w="100%">
        {usedGetCards.isLoading
          ? Array.from({ length: Math.ceil(PAGE_SIZE / columnCount) }).map((_, index) => (
              <Group gap={CARD_GAP} key={index} w={totalGridWidth} wrap="nowrap">
                {Array.from({ length: columnCount }).map((_, index) => (
                  <MtgCardButtonGhost key={index} />
                ))}
              </Group>
            ))
          : null}
        <Box h={first} mb={-CARD_GAP} />
        <>
          {virtualRows.map(item => {
            const rowStartIndex = item.index * columnCount

            return (
              <Group gap={CARD_GAP} key={item.key} w={totalGridWidth} wrap="nowrap">
                {Array.from({ length: columnCount }).map((_, columnIndex) => {
                  const card = cards[rowStartIndex + columnIndex]
                  if (!card)
                    return <Box {...CARD_BUTTON_DIMENSIONS} key={`${item.index}-${columnIndex}`} />

                  return <MtgCardButton card={card} key={card.oracle_id} />
                })}
              </Group>
            )
          })}
        </>
        <Box h={end} mt={-CARD_GAP} />
      </Stack>
      {showEmptyMessage && (
        <EmptyPlaceholder subText="Try refining your search." title="🤔 No cards found" />
      )}
      {showEndMessage && (
        <EmptyPlaceholder
          subText="The journey is complete, you may rest now 🛌."
          title="The end."
        />
      )}
    </Stack>
  )
}
