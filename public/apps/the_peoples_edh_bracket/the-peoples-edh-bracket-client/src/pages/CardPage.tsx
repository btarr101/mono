import { BarChart } from '@mantine/charts'
import {
  Alert,
  Anchor,
  Box,
  Button,
  Card,
  Center,
  Divider,
  Group,
  Indicator,
  NumberFormatter,
  NumberInput,
  Paper,
  Progress,
  Select,
  Stack,
  Text,
  Textarea,
  Title,
} from '@mantine/core'
import { hasLength, useForm } from '@mantine/form'
import { ArrowSquareOutIcon, InfoIcon, ShareIcon } from '@phosphor-icons/react'
import { parseAsStringLiteral, useQueryState } from 'nuqs'
import { Link, useLoaderData, useNavigate } from 'react-router'

import { LoadingImage } from '../components/LoadingImage'
import { Rating, RatingGhost } from '../components/Rating'
import { usePersonUUID } from '../hooks/useAuth'
import {
  useGetRatingHistogramForCard,
  useMyCardRating,
  usePatchRating,
  usePostRating,
  useRating,
  useRatings,
} from '../hooks/useRatings'
import type { CardRatingWithReviewsAndGlobalPoints } from '../types/bindings/CardRatingWithReviewsAndGlobalPoints'
import type { CardWithGlobalPoints } from '../types/bindings/CardWithGlobalPoints'
import { formatTimeStamp, safeNavigate } from '../util'

type SaveRatingParams = {
  points: number | null
  reason: string | null
}

export const CardPage = () => {
  const navigate = useNavigate()
  const { card } = useLoaderData<{ card: CardWithGlobalPoints }>()

  const personUUID = usePersonUUID()
  const [sort, setSort] = useQueryState(
    'sort',
    parseAsStringLiteral(['liked', 'disliked', 'controversial', 'recent'] as const).withDefault(
      'liked',
    ),
  )
  const { data: ratings, isPending: ratingsPending } = useRatings({
    card_oracle_id: card.oracle_id,
    rater_person_uuid: null,
    sort,
    page_size: 10,
  })
  const { data: myRating, isPending: myRatingPending } = useMyCardRating(card.oracle_id)
  const [pinned, setPinned] = useQueryState('pinned', {
    clearOnDefault: true,
  })
  const pinnedRating = useRating(pinned)

  const { mutateAsync: postRating } = usePostRating()
  const { mutateAsync: patchRating } = usePatchRating()
  const saveRating = async ({ points, reason }: SaveRatingParams) => {
    const pointsAndReason = {
      points: (points ?? 0.0).toString(),
      reason: reason || null,
    }

    await (myRating
      ? patchRating({
          uuid: myRating.uuid,
          ...pointsAndReason,
        })
      : postRating({
          card_oracle_id: card.oracle_id,
          ...pointsAndReason,
        }))
  }

  return (
    <Stack gap="xl" h="100dvh" justify="stretch" p="xl" w="100%">
      <Anchor w="fit-content" onClick={() => safeNavigate(navigate, -1, '/browse')}>
        {'<-'} Back
      </Anchor>
      <Group align="start">
        <CardSection card={card} />
        <InfoSection card={card} />
      </Group>
      {personUUID && (
        <Stack gap="sm">
          {myRatingPending ? (
            <RatingGhost />
          ) : (
            <>
              {!myRating && (
                <Alert
                  color="orange"
                  icon={<InfoIcon />}
                  title="You haven't rated this card yet."
                  variant="light"
                />
              )}
              <RatingInput rating={myRating ?? null} onSave={saveRating} />
            </>
          )}
        </Stack>
      )}
      <Stack pb="lg">
        <Group w={'100%'}>
          <Title order={1}>Community Ratings</Title>
          <Select
            allowDeselect={false}
            data={[
              {
                value: 'liked',
                label: '👍 Most Liked',
              },
              {
                value: 'disliked',
                label: '👎 Most Disliked',
              },
              {
                value: 'controversial',
                label: '🔥 Most Controversial',
              },
              {
                value: 'recent',
                label: '⏲️ Most Recent',
              },
            ]}
            defaultValue="liked"
            value={sort}
            onChange={newSort => setSort(newSort)}
          />
        </Group>
        <Stack gap="xl">
          {pinned &&
            (pinnedRating.isPending ? (
              <RatingGhost />
            ) : (
              pinnedRating.data && (
                <Rating
                  key={pinnedRating.data.uuid}
                  pinned={true}
                  rating={pinnedRating.data}
                  onPin={() => setPinned(null)}
                />
              )
            ))}
          {ratingsPending ? (
            <RatingGhost />
          ) : (
            ratings?.pages
              .flat()
              .flatMap(rating =>
                rating.rater_person_uuid === personUUID || rating.uuid === pinned
                  ? []
                  : [
                      <Rating
                        key={rating.uuid}
                        rating={rating}
                        onPin={() => setPinned(rating.uuid)}
                      />,
                    ],
              )
          )}
        </Stack>
      </Stack>
    </Stack>
  )
}

type CardSectionProps = {
  card: CardWithGlobalPoints
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
  card: CardWithGlobalPoints
}

const InfoSection = ({ card }: InfoSectionProps) => {
  const { data: buckets } = useGetRatingHistogramForCard(card.oracle_id, {
    buckets: 10,
  })

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
        <NumberFormatter decimalScale={2} suffix={' pts'} value={card.global_points} />
      </Title>
      <Stack align="end" gap={'xs'} w={'100%'}>
        <Text>Insta-Ban</Text>
        <Progress value={70} w={'100%'} />
      </Stack>
      <Title order={2}>Rank #46 Overall</Title>
      <Text c="dimmed">
        <NumberFormatter suffix={' ratings'} value={23} />
      </Text>
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
    </Stack>
  )
}

type RatingInputProps = {
  rating: CardRatingWithReviewsAndGlobalPoints | null
  onSave: (values: { points: number | null; reason: string | null }) => Promise<void>
}

const RatingInput = ({ rating, onSave }: RatingInputProps) => {
  const form = useForm({
    mode: 'controlled',
    initialValues: {
      points: rating ? Number(rating.points) : null, // todo: look into bigfloat impls
      reason: rating?.reason ?? '',
    },
    validate: {
      reason: hasLength({ max: 300 }, 'Reason must be less 300 characters or less'),
    },
  })

  return (
    <form
      onSubmit={form.onSubmit(async values => {
        await onSave(values)
        form.resetDirty(values)
      })}
    >
      <Indicator
        color="transparent"
        label={
          <Box pos={'relative'} w={0}>
            <Group
              pos="absolute"
              right={0}
              style={{ transform: 'translate(10%, -50%)' }}
              wrap="nowrap"
            >
              {rating !== null && (
                <Button.Group>
                  <Button
                    disabled
                    size="compact-md"
                    style={{ pointerEvents: 'none' }}
                    variant="default"
                  >
                    {rating.reviews.likes} 👍
                  </Button>
                  <Button
                    disabled
                    size="compact-md"
                    style={{ pointerEvents: 'none' }}
                    variant="default"
                  >
                    {rating.reviews.dislikes} 👎
                  </Button>
                  <Button size="compact-md" variant="default">
                    <ShareIcon />
                  </Button>
                </Button.Group>
              )}
              <Button
                disabled={!form.isDirty()}
                loading={form.submitting}
                miw={'fit-content'}
                type="submit"
              >
                Save
              </Button>
            </Group>
          </Box>
        }
        position="bottom-end"
        size={32}
      >
        <Card withBorder orientation="horizontal" padding="sm">
          <Card.Section withBorder mih={'125px'} p="md" style={{ alignSelf: 'stretch' }}>
            <Center h="100%">
              <Group wrap="nowrap">
                <Title order={2} textWrap="nowrap">
                  <NumberFormatter decimalScale={2} suffix={' pts'} value={rating?.global_points} />
                </Title>
                <Divider orientation="vertical" />
                <NumberInput
                  key={form.key('points')}
                  placeholder="0 ppts"
                  size="lg"
                  styles={{
                    input: {
                      fieldSizing: 'content',
                      paddingRight:
                        'calc(var(--input-right-section-width) + var(--mantine-spacing-md))',
                    },
                  }}
                  suffix=" ppts"
                  {...form.getInputProps('points')}
                />
              </Group>
            </Center>
          </Card.Section>
          <Card.Section flex={1}>
            <Textarea
              autosize
              key={form.key('reason')}
              placeholder="Enter a reason..."
              styles={{
                input: {
                  border: 'none',
                  borderRadius: 0,
                  fontSize: 'var(--mantine-font-size-md)',
                  padding: 'var(--mantine-spacing-md)',
                  resize: 'none',
                },
              }}
              w="100%"
              {...form.getInputProps('reason')}
            />
          </Card.Section>
        </Card>
      </Indicator>
      {rating && (
        <Text c="dimmed" px="xs" size="xs">
          {formatTimeStamp(rating.created_at)}
          {rating.updated_at && ' • edited'}
        </Text>
      )}
    </form>
  )
}
