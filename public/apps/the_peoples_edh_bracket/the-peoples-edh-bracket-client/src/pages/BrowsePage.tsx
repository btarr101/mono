import { Autocomplete, Flex, Group, Select, Stack } from '@mantine/core'
import { MagnifyingGlassIcon } from '@phosphor-icons/react'
import { useDebouncedValue } from '@tanstack/react-pacer'
import { parseAsStringLiteral, useQueryState } from 'nuqs'

import { MtgCardButton, MtgCardButtonGhost } from '../components/MtgCardButton'
import { useCards, useSearchCards } from '../hooks/useCards'

const PAGE_SIZE = 50

export const BrowsePage = () => {
  const [q, setQ] = useQueryState('q')
  const [debouncedQ] = useDebouncedValue(q, { wait: 500 })

  const [sort, setSort] = useQueryState(
    'sort',
    parseAsStringLiteral(['highest_rated', 'lowest_rated']),
  )

  const searchCards = useSearchCards(q || null)
  const { data, isLoading, isFetching } = useCards({
    q: debouncedQ || null,
    sort,
    page_size: PAGE_SIZE,
  })

  return (
    <Stack h="100dvh" p="xl" w="100%">
      <Group w={'100%'}>
        <Autocomplete
          data={searchCards.data?.pages.flat().map(({ name }) => name)}
          filter={({ options }) => options}
          loading={isFetching}
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
              label: 'Highest Rated',
            },
            {
              value: 'lowest_rated',
              label: 'Lowest Rated',
            },
          ]}
          placeholder="sort by"
          value={sort}
          onChange={newSort => setSort(newSort)}
        />
      </Group>
      <Flex gap={'lg'} justify={'center'} wrap={'wrap'}>
        {isLoading
          ? Array.from({ length: PAGE_SIZE }).map((_, index) => <MtgCardButtonGhost key={index} />)
          : data?.pages.flat().map(card => <MtgCardButton card={card} key={card.oracle_id} />)}
      </Flex>
    </Stack>
  )
}
