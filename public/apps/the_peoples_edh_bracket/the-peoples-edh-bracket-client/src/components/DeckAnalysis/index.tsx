import { BarChart } from '@mantine/charts'
import { Group, Stack, Text, Title } from '@mantine/core'

import { PointsNumberFormatter } from '../../components/PointsNumberFormatter'
import type { AnalyzedDeckWithSource } from '../../types/bindings/AnalyzedDeckWithSource'
import { type DeckSource, DeckSourceButton } from './DeckSourceButton'
import { DeckTable } from './DeckTable'

export type DeckAnalysisProps = {
  analyzedDeck: AnalyzedDeckWithSource
}

export const DeckAnalysis = ({ analyzedDeck }: DeckAnalysisProps) => {
  const barChartData = analyzedDeck.histogram.map(
    ({ lower_inclusive_points_bound, upper_exclusive_points_bound, count }) => ({
      pts: (Number(lower_inclusive_points_bound) + Number(upper_exclusive_points_bound)) / 2,
      cards: count,
      window: `${lower_inclusive_points_bound} - ${upper_exclusive_points_bound} pts`,
    }),
  )

  const source =
    analyzedDeck.url_source !== null
      ? {
          ty: 'url' as const,
          url: analyzedDeck.url_source,
        }
      : ({
          ty: 'decklist' as const,
          deck: analyzedDeck.deck,
        } satisfies DeckSource)

  return (
    <Stack>
      <Group align="stretch" wrap="nowrap">
        <Stack h={320}>
          <DeckSourceButton source={source} />
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
