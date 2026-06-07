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
import type { CardRating } from '../types/bindings/CardRating'
import { PersonProfileLine } from './PersonProfileLine'

export type RatingProps = {
  rating: CardRating
  pinned?: boolean
  onPin?: () => void
}

export const Rating = ({ rating, pinned, onPin }: RatingProps) => {
  const person = usePerson(rating.rater_person_uuid)

  return (
    <Box pos="relative">
      <Indicator
        color="transparent"
        label={
          <Box pos={'relative'} w={0}>
            <Button.Group pos="absolute" right={0} style={{ transform: 'translate(10%, -50%)' }}>
              <Button size="compact-md" variant="light">
                10 👍
              </Button>
              <Button size="compact-md" variant="default">
                5 👎
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
                    <NumberFormatter suffix={' pts'} value={10} />
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
    </Box>
  )
}

export const RatingGhost = () => <Skeleton h={127} w="100%" />
