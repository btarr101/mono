import { Autocomplete, Flex, Group, Select, Stack } from '@mantine/core'
import { MagnifyingGlassIcon } from '@phosphor-icons/react'
import { useDebouncedValue } from '@tanstack/react-pacer'
import { parseAsStringLiteral, useQueryState } from 'nuqs'

import { MtgCardButton } from '../components/MtgCardButton'
import { useCards, useSearchCards } from '../hooks/useCards'

export const BrowsePage = () => {
  const [q, setQ] = useQueryState('q')
  const [debouncedQ] = useDebouncedValue(q, { wait: 500 })

  const [sort, setSort] = useQueryState(
    'sort',
    parseAsStringLiteral(['highest_rated', 'lowest_rated']),
  )

  const { data: allCards, isLoading: allCardsLoading } = useCards({
    q: q || null,
    sort,
    page_size: 10,
  })
  const cards = useSearchCards(debouncedQ || null)

  return (
    <Stack h="100dvh" p="xl" w="100%">
      <Group w={'100%'}>
        <Autocomplete
          data={allCards?.pages.flat().map(({ name }) => name)}
          filter={({ options }) => options}
          loading={allCardsLoading}
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
        {cards.data?.pages.flat().map((card, index) => (
          <MtgCardButton card={card} key={index} />
        ))}
      </Flex>
    </Stack>
  )
}
