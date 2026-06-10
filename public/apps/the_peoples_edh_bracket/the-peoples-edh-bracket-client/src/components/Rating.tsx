import {
  Box,
  Button,
  Card,
  Center,
  Divider,
  Group,
  Indicator,
  Menu,
  NumberFormatter,
  Skeleton,
  Stack,
  Text,
  Title,
} from '@mantine/core'
import { PushPinIcon, ShareIcon } from '@phosphor-icons/react'

import { usePerson } from '../hooks/usePersons'
import { usePostReviewRating } from '../hooks/useRatings'
import type { CardRatingWithReviewsAndGlobalPoints } from '../types/bindings/CardRatingWithReviewsAndGlobalPoints'
import { formatTimeStamp } from '../util'
import { PersonProfileLine } from './PersonProfileLine'

export type RatingProps = {
  rating: CardRatingWithReviewsAndGlobalPoints
  pinned?: boolean
  onPin?: () => void
}

export const Rating = ({ rating, pinned, onPin }: RatingProps) => {
  const person = usePerson(rating.rater_person_uuid)
  const { mutate: reviewRating } = usePostReviewRating()

  const personLiked = rating.reviews.person_review === true
  const personDisliked = rating.reviews.person_review === false

  const onLike = () =>
    reviewRating({
      uuid: rating.uuid,
      like: personLiked ? null : true,
    })

  const onDislike = () =>
    reviewRating({
      uuid: rating.uuid,
      like: personDisliked ? null : false,
    })

  return (
    <Box pos="relative">
      <Indicator
        color="transparent"
        label={
          <Box pos={'relative'} w={0}>
            <Button.Group pos="absolute" right={0} style={{ transform: 'translate(10%, -50%)' }}>
              <Button
                size="compact-md"
                variant={personLiked ? 'light' : 'default'}
                onClick={onLike}
              >
                {rating.reviews.likes} 👍
              </Button>
              <Button
                size="compact-md"
                variant={rating.reviews.person_review === false ? 'light' : 'default'}
                onClick={onDislike}
              >
                {rating.reviews.dislikes} 👎
              </Button>
              <Button size="compact-md" variant="default">
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
      >
        <Card withBorder orientation="horizontal" padding="sm">
          <Card.Section withBorder p="md">
            <Center h="100%">
              <Stack align="start" gap={'lg'}>
                <Group wrap="nowrap">
                  <Title order={2} textWrap="nowrap">
                    <NumberFormatter
                      decimalScale={2}
                      suffix={' pts'}
                      value={rating.global_points}
                    />
                  </Title>
                  <Divider orientation="vertical" />
                  <Title c="dimmed" order={4} textWrap="nowrap">
                    <NumberFormatter suffix={' ppts'} value={rating.points} />
                  </Title>
                </Group>
                <PersonProfileLine loading={person.isLoading} person={person.data}>
                  <Menu.Item>View Profile</Menu.Item>
                </PersonProfileLine>
              </Stack>
            </Center>
          </Card.Section>
          <Card.Section p="md">
            <Text>{rating.reason ?? ''}</Text>
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

export const RatingGhost = () => <Skeleton h={127} w="100%" />
