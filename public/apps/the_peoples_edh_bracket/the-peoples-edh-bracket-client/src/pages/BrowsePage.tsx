import { Autocomplete, Flex, Group, Select, Stack } from '@mantine/core'
import { MagnifyingGlassIcon } from '@phosphor-icons/react'

import { MtgCardButton } from '../components/MtgCardButton'

const TEMP_CARDS = ['Black Lotus', 'Storm Crow', 'Eddymurk Crab']

export const BrowsePage = () => {
  return (
    <Stack h="100dvh" p="xl" w="100%">
      <Group w={'100%'}>
        <Autocomplete
          data={TEMP_CARDS}
          placeholder="Search for a card..."
          rightSection={<MagnifyingGlassIcon />}
          style={{
            flex: 1,
          }}
        />
        <Select data={['Trending', 'Highest Rated', 'Lowest Rated']} placeholder="sort by" />
      </Group>
      <Flex gap={'lg'} justify={'center'} wrap={'wrap'}>
        {Array.from({ length: 30 }).map((_, index) => (
          <MtgCardButton key={index} />
        ))}
      </Flex>
    </Stack>
  )
}
