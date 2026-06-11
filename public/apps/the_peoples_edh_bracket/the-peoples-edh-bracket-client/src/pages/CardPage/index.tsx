import { BarChart } from '@mantine/charts'
import {
  Anchor,
  Box,
  Button,
  Center,
  Group,
  LoadingOverlay,
  NumberFormatter,
  Paper,
  Progress,
  Stack,
  Text,
  Title,
} from '@mantine/core'
import { ArrowSquareOutIcon } from '@phosphor-icons/react'
import { Link, useLoaderData, useNavigate } from 'react-router'

import { LoadingImage } from '../../components/LoadingImage'
import { useGetCardMetrics } from '../../hooks/useCards'
import { useGetRatingHistogramForCard } from '../../hooks/useRatings'
import type { Card } from '../../types/bindings/Card'
import type { CardWithMetrics } from '../../types/bindings/CardWithMetrics'
import { safeNavigate } from '../../util'
import { RatingSection } from './RatingSection'

export const CardPage = () => {
  const navigate = useNavigate()
  const { card } = useLoaderData<{ card: CardWithMetrics }>()

  return (
    <Stack gap="xl" h="100dvh" justify="stretch" p="xl" w="100%">
      <Anchor w="fit-content" onClick={() => safeNavigate(navigate, -1, '/browse')}>
        {'<-'} Back
      </Anchor>
      <Group align="start">
        <CardSection card={card} />
        <InfoSection card={card} />
      </Group>
      <RatingSection cardOracleId={card.oracle_id} />
    </Stack>
  )
}

type CardSectionProps = {
  card: Card
}

const CardSection = ({ card }: CardSectionProps) => (
  <Stack align="center" flex={1} h="100%" miw="fit-content">
    <Paper
      radius={'15px'}
      shadow="lg"
      style={{
        overflow: 'clip',
      }}
    >
      <LoadingImage
        src={
          card.image_uri ||
          'https://cards.scryfall.io/large/front/0/3/036ef8c9-72ac-46ce-af07-83b79d736538.jpg?1562730661'
        }
        w={320}
      />
    </Paper>
    <Center flex={1}>
      <Button
        component={Link}
        rightSection={<ArrowSquareOutIcon />}
        target="_blank"
        to="https://scryfall.com/card/9ed/100/storm-crow"
        w={'fit-content'}
      >
        View on Scryfall
      </Button>
    </Center>
  </Stack>
)

type InfoSectionProps = {
  card: CardWithMetrics
}

const InfoSection = ({ card }: InfoSectionProps) => {
  const usedGetCardMetrics = useGetCardMetrics(card.oracle_id)
  const { data: buckets } = useGetRatingHistogramForCard(card.oracle_id, {
    buckets: 10,
  })

  const cardPoints = usedGetCardMetrics.data?.global_points ?? card.global_points
  const totalRatings = usedGetCardMetrics.data?.total_ratings ?? card.total_ratings

  const barChartData = buckets?.map(
    ({ lower_inclusive_points_bound, upper_exclusive_points_bound, count }) => ({
      pts: (Number(lower_inclusive_points_bound) + Number(upper_exclusive_points_bound)) / 2,
      ratings: count,
      window: `${lower_inclusive_points_bound} - ${upper_exclusive_points_bound} pts`,
    }),
  )

  return (
    <Stack flex={3} h={'100%'}>
      <Title order={1}>{card.name}</Title>

      <Text c="dimmed" maw={540} size="xl">
        Community Power Score
      </Text>
      <Title order={1} textWrap="nowrap">
        <NumberFormatter decimalScale={2} suffix={' pts'} value={cardPoints ?? '..'} />
      </Title>
      <Progress value={Number(cardPoints) * 10.0} w={'100%'} />
      <Title order={2}>Rank #46 Overall</Title>
      <Text c="dimmed">
        <NumberFormatter suffix={' ratings'} value={totalRatings} />
      </Text>
      <Box pos="relative">
        <LoadingOverlay visible={!barChartData?.length} />
        <BarChart
          data={barChartData ?? []}
          dataKey="pts"
          h={240}
          series={[{ name: 'ratings', color: 'var(--mantine-primary-color-filled)' }]}
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
      </Box>
    </Stack>
  )
}
