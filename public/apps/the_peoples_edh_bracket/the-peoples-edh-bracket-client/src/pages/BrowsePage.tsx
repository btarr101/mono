import { Autocomplete, Flex, Group, Select, Stack } from '@mantine/core'
import { MagnifyingGlassIcon } from '@phosphor-icons/react'
import { parseAsStringLiteral, useQueryState } from 'nuqs'

import { MtgCardButton, MtgCardButtonGhost } from '../components/MtgCardButton'
import { useDebouncedSearchCards, useGetCards } from '../hooks/useCards'
import type { GetCardsParamsSort } from '../types/bindings/GetCardsParamsSort'

const PAGE_SIZE = 50

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
  const usedGetCards = useGetCards({
    q: debouncedQ,
    sort,
    page_size: PAGE_SIZE,
  })

  const isAutocompleteLoading = isDebouncing || usedSearchCards.isFetching

  return (
    <Stack h="100dvh" p="xl" w="100%">
      <Group w={'100%'}>
        <Autocomplete
          data={
            isAutocompleteLoading
              ? [
                  {
                    value: '...',
                    disabled: true,
                  },
                ]
              : (usedSearchCards.data?.pages.flat().map(({ name }) => name) ?? [])
          }
          filter={({ options }) => options}
          loading={usedGetCards.isFetching}
          placeholder="Search for a card..."
          rightSection={<MagnifyingGlassIcon />}
          style={{
            flex: 1,
          }}
          value={q ?? ''}
          onChange={newValue => setQ(newValue ?? undefined)}
        />
        <Select
          data={[
            {
              value: 'highest_rated',
              label: '👑 Highest Rated',
            },
            {
              value: 'lowest_rated',
              label: '🗑️ Lowest Rated',
            },
            {
              value: 'most_controversial',
              label: '⚔️ Most Controversial',
            },
            {
              value: 'most_rated',
              label: '👀 Most Rated',
            },
            {
              value: 'least_rated',
              label: '👻 Least Rated',
            },
            {
              value: 'trending',
              label: '🔥 Trending',
            },
          ]}
          placeholder="sort by"
          value={sort}
          onChange={newSort => setSort(newSort)}
        />
      </Group>
      <Flex gap={'lg'} justify={'center'} wrap={'wrap'}>
        {usedGetCards.isLoading
          ? Array.from({ length: PAGE_SIZE }).map((_, index) => <MtgCardButtonGhost key={index} />)
          : usedGetCards.data?.pages
              .flat()
              .map(card => <MtgCardButton card={card} key={card.oracle_id} />)}
      </Flex>
    </Stack>
  )
}
