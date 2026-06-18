import {
  Anchor,
  Text,
  Button,
  Center,
  Group,
  NumberFormatter,
  Stack,
  TextInput,
  Title,
  Paper,
  Autocomplete,
  Select,
  Table,
} from '@mantine/core'
import { Link, useLoaderData, useNavigate } from 'react-router'
import { safeNavigate } from '../../util'
import { BarChart } from '@mantine/charts'
import type { RatingHistogramBucket } from '../../types/bindings/RatingHistogramBucket'
import { FilesIcon, MagnifyingGlassIcon } from '@phosphor-icons/react'
import { useState } from 'react'
import type { AnalyzedDeck } from '../../types/bindings/AnalyzedDeck'
import { LoadingImage } from '../../components/LoadingImage'
import type { CardWithGlobalPoints } from '../../types/bindings/CardWithGlobalPoints'
import type { DeckMaindeckEntry } from '../../types/bindings/DeckMaindeckEntry'

const TEST_DATA = [
  {
    lower_inclusive_points_bound: '0',
    upper_exclusive_points_bound: '1',
    count: 5,
  },
  {
    lower_inclusive_points_bound: '1',
    upper_exclusive_points_bound: '2',
    count: 3,
  },
  {
    lower_inclusive_points_bound: '2',
    upper_exclusive_points_bound: '3',
    count: 40,
  },
  {
    lower_inclusive_points_bound: '3',
    upper_exclusive_points_bound: '4',
    count: 30,
  },
  {
    lower_inclusive_points_bound: '4',
    upper_exclusive_points_bound: '5',
    count: 12,
  },
  {
    lower_inclusive_points_bound: '5',
    upper_exclusive_points_bound: '6',
    count: 9,
  },
  {
    lower_inclusive_points_bound: '6',
    upper_exclusive_points_bound: '7',
    count: 0,
  },
  {
    lower_inclusive_points_bound: '7',
    upper_exclusive_points_bound: '8',
    count: 5,
  },
  {
    lower_inclusive_points_bound: '8',
    upper_exclusive_points_bound: '9',
    count: 3,
  },
  {
    lower_inclusive_points_bound: '9',
    upper_exclusive_points_bound: '10',
    count: 6,
  },
] satisfies RatingHistogramBucket[]

export const AnalayzeNewDeckPage = () => {
  const { newAnalyzedDeck } = useLoaderData<{ newAnalyzedDeck: AnalyzedDeck }>()

  return <AnalyzedDeckPageComponent analyzedDeck={newAnalyzedDeck} />
}

export type AnalayzedDeckPageComponentProps = {
  analyzedDeck: AnalyzedDeck
}

const sortOptions = ['highest_rated', 'lowest_rated'] as const
type SortOption = (typeof sortOptions)[number]

type CardEntry =
  | {
      ty: 'commander'
      card: CardWithGlobalPoints
    }
  | ({
      ty: 'maindeck-entry'
    } & DeckMaindeckEntry)

export const AnalyzedDeckPageComponent = ({ analyzedDeck }: AnalayzedDeckPageComponentProps) => {
  const navigate = useNavigate()

  const [q, setQ] = useState<string | null>(null)
  const [sort, setSort] = useState<SortOption | null>(null)

  const barChartData = TEST_DATA?.map(
    ({ lower_inclusive_points_bound, upper_exclusive_points_bound, count }) => ({
      pts: (Number(lower_inclusive_points_bound) + Number(upper_exclusive_points_bound)) / 2,
      cards: count,
      window: `${lower_inclusive_points_bound} - ${upper_exclusive_points_bound} pts`,
    }),
  )

  const deck = analyzedDeck.deck
  const commanderEntries = deck.commanders.map(
    commander =>
      ({
        ty: 'commander',
        card: commander,
      }) satisfies CardEntry,
  )
  const maindeckEntries = deck.maindeck.map(
    entry => ({ ty: 'maindeck-entry', ...entry }) satisfies CardEntry,
  )

  const sortCompareFunc = (a: CardEntry, b: CardEntry) => {
    if (sort !== undefined) {
      const difference = parseFloat(a.card.global_points) - parseFloat(b.card.global_points)
      if (difference < 0) return Number(sort === 'highest_rated') * 2 - 1
      if (difference > 0) return 1 - Number(sort === 'highest_rated') * 2
    }

    return a.card.name.localeCompare(b.card.name)
  }

  const sortedCardEntries = [
    ...commanderEntries.toSorted(sortCompareFunc),
    ...maindeckEntries.toSorted(sortCompareFunc),
  ]

  const cardNames = sortedCardEntries.map(({ card: { name } }) => name)
  const filteredEntries = sortedCardEntries.filter(entry =>
    entry.card.name.toLowerCase().startsWith(q?.toLowerCase() ?? ''),
  )

  return (
    <Stack gap="xl" mih="100dvh" justify="stretch" p="xl" w="100%">
      <Anchor w="fit-content" onClick={() => safeNavigate(navigate, -1, '/analyze')}>
        {'<-'} Back
      </Anchor>
      <Group align="start" justify="space-between">
        <TextInput flex={1} size="lg" placeholder="Untitled Deck" />
        <Button size="lg">Save to tracked Decks</Button>
      </Group>
      <Group wrap="nowrap" align="stretch">
        <Stack h={320}>
          <Paper withBorder h={'100%'} flex={1}>
            <Center h="100%">
              <Stack>
                <Group wrap="nowrap">
                  <FilesIcon size={32} />
                  <Text>Source: Decklist</Text>
                </Group>
                <Button>Edit</Button>
              </Stack>
            </Center>
          </Paper>
          <Text c="dimmed" maw={540} size="xl">
            Total Points
          </Text>
          <Title size={48} textWrap="nowrap">
            <NumberFormatter decimalScale={2} fixedDecimalScale value={100} suffix=" pts" />
          </Title>
        </Stack>
        <BarChart
          data={barChartData}
          series={[{ name: 'cards', color: 'var(--mantine-primary-color-filled)' }]}
          dataKey="pts"
          tickLine="x"
          tooltipProps={{
            labelFormatter: (_, payload) => payload?.[0]?.payload?.window ?? '',
          }}
          h={320}
          xAxisProps={{
            domain: [0, 10],
            ticks: Array.from({ length: 11 }, (_, i) => i),
            type: 'number',
            unit: ' pts',
          }}
          yAxisProps={{
            allowDecimals: false,
          }}
        />
      </Group>
      <Group w={'100%'}>
        <Autocomplete
          data={cardNames}
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
          ]}
          placeholder="sort by"
          value={sort}
          onChange={newSort => setSort(newSort)}
        />
      </Group>
      <Table verticalSpacing={4}>
        <colgroup>
          <col />
          <col />
          <col />
          <col style={{ width: '100%' }} />
        </colgroup>
        <Table.Thead>
          <Table.Tr>
            <Table.Th>Count</Table.Th>
            <Table.Th>Card</Table.Th>
            <Table.Th>pts</Table.Th>
            <Table.Th ta="right" />
          </Table.Tr>
        </Table.Thead>
        <Table.Tbody>
          {filteredEntries.map(entry => (
            <Table.Tr key={entry.card.oracle_id}>
              <Table.Td>{entry.ty === 'commander' ? 1 : entry.count}</Table.Td>
              <Table.Td
                style={{
                  whiteSpace: 'nowrap',
                }}
              >
                <Group wrap="nowrap">
                  <LoadingImage src={entry.card.image_uri} w={32} />
                  {entry.ty === 'commander' && '👑 '}
                  {entry.card.name}
                </Group>
              </Table.Td>
              <Table.Td
                style={{
                  textWrap: 'nowrap',
                }}
              >
                <NumberFormatter
                  value={entry.card.global_points}
                  decimalScale={2}
                  fixedDecimalScale
                  suffix=" pts"
                />
              </Table.Td>
              <Table.Td ta="right">
                <Button component={Link} to={{ pathname: `/browse/${entry.card.oracle_id}` }}>
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
