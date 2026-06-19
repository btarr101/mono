import { BarChart } from '@mantine/charts'
import { Anchor, Button, Group, Stack, Text, TextInput, Title } from '@mantine/core'
import { useLoaderData, useNavigate } from 'react-router'

import { PointsNumberFormatter } from '../../components/PointsNumberFormatter'
import type { AnalyzedDeck } from '../../types/bindings/AnalyzedDeck'
import { safeNavigate } from '../../util'
import { DeckSource } from './DeckSource'
import { DeckTable } from './DeckTable'

export const AnalayzeNewDeckPage = () => {
  const { newAnalyzedDeck } = useLoaderData<{ newAnalyzedDeck: AnalyzedDeck }>()

  return <AnalyzedDeckPageComponent analyzedDeck={newAnalyzedDeck} />
}

export type AnalayzedDeckPageComponentProps = {
  analyzedDeck: AnalyzedDeck
}

export const AnalyzedDeckPageComponent = ({ analyzedDeck }: AnalayzedDeckPageComponentProps) => {
  const navigate = useNavigate()

  const barChartData = analyzedDeck.histogram.map(
    ({ lower_inclusive_points_bound, upper_exclusive_points_bound, count }) => ({
      pts: (Number(lower_inclusive_points_bound) + Number(upper_exclusive_points_bound)) / 2,
      cards: count,
      window: `${lower_inclusive_points_bound} - ${upper_exclusive_points_bound} pts`,
    }),
  )

  return (
    <Stack gap="xl" justify="stretch" mih="100dvh" p="xl" w="100%">
      <Anchor w="fit-content" onClick={() => safeNavigate(navigate, -1, '/analyze')}>
        {'<-'} Back
      </Anchor>
      <Group align="start" justify="space-between">
        <TextInput flex={1} placeholder="Enter deck name..." size="lg" />
        <Button size="lg">Save to tracked Decks</Button>
      </Group>
      <Group align="stretch" wrap="nowrap">
        <Stack h={320}>
          <DeckSource source={analyzedDeck.source} />
          <Text c="dimmed" maw={540} size="xl">
            Total
          </Text>
          <Title size={48} textWrap="nowrap">
            <PointsNumberFormatter points={analyzedDeck.total_points} suffix=" pts" />
          </Title>
        </Stack>
        <BarChart
          data={barChartData}
          dataKey="pts"
          h={320}
          series={[{ name: 'cards', color: 'var(--mantine-primary-color-filled)' }]}
          tickLine="x"
          tooltipProps={{
            labelFormatter: (_, payload) => payload?.[0]?.payload?.window ?? '',
          }}
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
      <DeckTable deck={analyzedDeck.deck} />
    </Stack>
  )
}
