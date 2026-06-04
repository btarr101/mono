import { Autocomplete, Grid, Group, Select, Stack } from '@mantine/core'
import { MagnifyingGlassIcon } from '@phosphor-icons/react'

import { MtgCardButton } from '../components/MtgCardButton'

const TEMP_CARDS = ['Black Lotus', 'Storm Crow', 'Eddymurk Crab']

export const BrowsePage = () => {
  return (
    <Stack h="100dvh" justify="stretch" p="xl" w="100%">
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
      <Grid>
        {Array.from({ length: 30 }).map((_, index) => (
          <Grid.Col key={index} span={12 / 5}>
            <MtgCardButton />
          </Grid.Col>
        ))}
      </Grid>
    </Stack>
  )
}
