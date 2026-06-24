import 'react-virtualized/styles.css'

import { Autocomplete, Box, Flex, Group, Select, Stack } from '@mantine/core'
import { MagnifyingGlassIcon } from '@phosphor-icons/react'
import { useWindowVirtualizer } from '@tanstack/react-virtual'
import { parseAsStringLiteral, useQueryState } from 'nuqs'
import { AutoSizer, Grid, WindowScroller } from 'react-virtualized'

import { EmptyPlaceholder } from '../components/EmptyPlaceholder'
import { MtgCardButton, MtgCardButtonGhost } from '../components/MtgCardButton'
import { CARD_BUTTON_DIMENSIONS } from '../components/MtgCardButton.constants'
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

      {usedGetCards.isLoading ? (
        <Flex gap="lg" justify="center" wrap="wrap">
          {Array.from({ length: PAGE_SIZE }).map((_, index) => (
            <MtgCardButtonGhost key={index} />
          ))}
        </Flex>
      ) : (
        <WindowScroller>
          {({ height, isScrolling, onChildScroll, scrollTop, registerChild }) => (
            <AutoSizer disableHeight>
              {({ width }) => {
                const columnCount = Math.max(
                  1,
                  Math.floor(width / (CARD_BUTTON_DIMENSIONS.w + CARD_GAP)),
                )
                const rowCount = Math.ceil(cards.length / columnCount)
                const totalGridWidth = columnCount * (CARD_BUTTON_DIMENSIONS.w + CARD_GAP)
                const paddingLeft = Math.max(0, (width - totalGridWidth) / 2)

                return (
                  <Box pl={paddingLeft} ref={registerChild}>
                    <Grid
                      autoHeight
                      cellRenderer={({ columnIndex, rowIndex, key, style }) => {
                        const cardIndex = rowIndex * columnCount + columnIndex
                        const card = cards[cardIndex]
                        if (!card) return undefined

                        return (
                          <Box
                            key={key}
                            style={{
                              ...style,
                              display: 'flex',
                              alignItems: 'center',
                              justifyContent: 'center',
                              padding: CARD_GAP / 2,
                              boxSizing: 'border-box',
                            }}
                          >
                            <MtgCardButton card={card} />
                          </Box>
                        )
                      }}
                      columnCount={columnCount}
                      columnWidth={CARD_BUTTON_DIMENSIONS.w + CARD_GAP}
                      height={height}
                      isScrolling={isScrolling}
                      overscanRowCount={2}
                      rowCount={rowCount}
                      rowHeight={CARD_BUTTON_DIMENSIONS.h + CARD_GAP}
                      scrollTop={scrollTop}
                      width={totalGridWidth}
                      onScroll={onChildScroll}
                      onSectionRendered={({ rowStopIndex }) => {
                        if (
                          rowStopIndex >= rowCount - 1 &&
                          usedGetCards.hasNextPage &&
                          !usedGetCards.isFetching
                        ) {
                          usedGetCards.fetchNextPage()
                        }
                      }}
                    />
                  </Box>
                )
              }}
            </AutoSizer>
          )}
        </WindowScroller>
      )}
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
