import {
  Box,
  Button,
  Card,
  Center,
  Divider,
  Group,
  Indicator,
  Skeleton,
  Stack,
  Text,
  Title,
} from '@mantine/core'
import { PushPinIcon, ShareIcon } from '@phosphor-icons/react'

import { useMe, usePerson } from '../hooks/usePersons'
import { usePutRatingReview } from '../hooks/useRatings'
import type { CardRatingEnriched } from '../types/bindings/CardRatingEnriched'
import { formatTimeStamp } from '../util'
import { PointsNumberFormatter } from './PointsNumberFormatter'
import { ViewablePersonProfileLine } from './ViewablePersonProfileLine'

export type RatingProps = {
  rating: CardRatingEnriched
  pinned?: boolean
  onPin?: () => void
  onShare?: () => void
}

export const Rating = ({ rating, pinned, onPin, onShare }: RatingProps) => {
  const person = usePerson(rating.rater_person_uuid)
  const me = useMe()
  const loggedInPersonUUID = me.data?.uuid ?? null
  const { mutate: reviewRating } = usePutRatingReview()

  const personLiked = rating.reviews.person_review === true
  const personDisliked = rating.reviews.person_review === false

  const onLike = () => {
    reviewRating({
      uuid: rating.uuid,
      like: personLiked ? null : true,
    })
  }

  const onDislike = () => {
    reviewRating({
      uuid: rating.uuid,
      like: personDisliked ? null : false,
    })
  }

  return (
    <Box pos="relative">
      <Indicator
        color="transparent"
        label={
          <Box pos={'relative'} w={0}>
            <Button.Group pos="absolute" right={0} style={{ transform: 'translate(10%, -50%)' }}>
              <Button
                disabled={
                  loggedInPersonUUID === null || loggedInPersonUUID === rating.rater_person_uuid
                }
                size="compact-md"
                variant={personLiked ? 'light' : 'default'}
                onClick={onLike}
              >
                {rating.reviews.likes} 👍
              </Button>
              <Button
                disabled={
                  loggedInPersonUUID === null || loggedInPersonUUID === rating.rater_person_uuid
                }
                size="compact-md"
                variant={rating.reviews.person_review === false ? 'light' : 'default'}
                onClick={onDislike}
              >
                {rating.reviews.dislikes} 👎
              </Button>
              <Button size="compact-md" variant="default" onClick={onShare}>
                <ShareIcon />
              </Button>
              <Button size="compact-md" variant={pinned ? 'filled' : 'default'} onClick={onPin}>
                <PushPinIcon />
              </Button>
            </Button.Group>
          </Box>
        }
        position="bottom-end"
        size={32}
        zIndex={1}
      >
        <Card withBorder orientation="horizontal" padding="sm">
          <Card.Section withBorder p="md">
            <Center h="100%">
              <Stack align="start" gap={'lg'}>
                <Group wrap="nowrap">
                  <Title order={2} textWrap="nowrap">
                    <PointsNumberFormatter points={rating.global_points} suffix=" pts" />
                  </Title>
                  <Divider orientation="vertical" />
                  <Stack gap={0}>
                    <Title c="dimmed" order={4} textWrap="nowrap">
                      <PointsNumberFormatter points={rating.points} suffix=" ppts" />
                    </Title>
                    <Divider />
                    <Text span c="dimmed" size="sm">
                      <PointsNumberFormatter points={rating.total_points} suffix=" ppts" />
                    </Text>
                  </Stack>
                </Group>
                <ViewablePersonProfileLine loading={person.isLoading} person={person.data} />
              </Stack>
            </Center>
          </Card.Section>
          <Card.Section p="md">
            {rating.reason ? (
              <Text>{rating.reason}</Text>
            ) : (
              <Text c="dimmed">*No reason provided</Text>
            )}
          </Card.Section>
        </Card>
      </Indicator>
      <Text c="dimmed" px="xs" size="xs">
        {formatTimeStamp(rating.created_at)}
        {rating.updated_at && ' • edited'}
      </Text>
    </Box>
  )
}

export const RatingGhost = () => <Skeleton h={144} w="100%" />
