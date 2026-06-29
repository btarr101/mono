import { Avatar, Button, Group, Stack, Tabs, Text, Title } from '@mantine/core'
import { parseAsStringLiteral, useQueryState } from 'nuqs'
import { useLoaderData, useRevalidator } from 'react-router'

import { BackAnchor } from '../../components/BackAnchor'
import { PointsNumberFormatter } from '../../components/PointsNumberFormatter'
import { Stat } from '../../components/Stat'
import { useFollowPerson, useUnfollowPerson } from '../../hooks/usePersons'
import type { PersonEnriched } from '../../types/bindings/PersonEnriched'
import { formatTimeStamp } from '../../util'
import { FolloweesPanelContent } from './FolloweesPanelContent'
import { FollowersPanelContent } from './FollowersPanelContent'
import { RatingsPanelContent } from './RatingsPanelContent'
import { TrackedDecksPanelContent } from './TrackedDecksPanelContent'

const TABS = ['ratings', 'decks', 'followers', 'followees']

export const ProfilePage = () => {
  const { person } = useLoaderData<{ person: PersonEnriched }>()
  const [tab, setTab] = useQueryState(
    'tab',
    parseAsStringLiteral(TABS).withDefault('ratings').withOptions({
      clearOnDefault: false,
    }),
  )

  return (
    <Stack gap="xl" justify="stretch" mih="100vh" p="xl" w="100%">
      <BackAnchor fallback="/browse" />
      <HeadSection person={person} />
      <Tabs
        value={tab}
        variant="outline"
        onChange={rawValue => {
          const newTab = TABS.find(option => option === rawValue)
          if (newTab) setTab(newTab)
        }}
      >
        <Tabs.List>
          <Tabs.Tab value="ratings">
            <Text size="xl">Ratings</Text>
          </Tabs.Tab>
          <Tabs.Tab value="decks">
            <Text size="xl">Tracked Decks</Text>
          </Tabs.Tab>
          <Tabs.Tab value="followers">
            <Text size="xl">Followers</Text>
          </Tabs.Tab>
          <Tabs.Tab value="followees">
            <Text size="xl">Followees</Text>
          </Tabs.Tab>
        </Tabs.List>
        <Tabs.Panel value="ratings">
          <RatingsPanelContent personUUID={person.uuid} />
        </Tabs.Panel>
        <Tabs.Panel value="decks">
          <TrackedDecksPanelContent personUUID={person.uuid} />
        </Tabs.Panel>
        <Tabs.Panel value="followers">
          <FollowersPanelContent personUUID={person.uuid} />
        </Tabs.Panel>
        <Tabs.Panel value="followees">
          <FolloweesPanelContent personUUID={person.uuid} />
        </Tabs.Panel>
      </Tabs>
    </Stack>
  )
}

type HeadSectionProps = {
  person: PersonEnriched
}

export const HeadSection = ({ person }: HeadSectionProps) => {
  const { revalidate } = useRevalidator()
  const { mutateAsync: follow } = useFollowPerson()
  const { mutateAsync: unfollow } = useUnfollowPerson()

  return (
    <Stack gap="xl">
      <Group align="stretch">
        <Avatar imageProps={{ referrerPolicy: 'no-referrer' }} size="96" src={person.picture_url} />
        <Stack gap="xs" h="100%">
          <Group>
            <Title size="2rem" textWrap="nowrap">
              {person.username}
            </Title>
            {person.am_following !== null &&
              (person.am_following ? (
                <Button onClick={() => unfollow(person.uuid).then(revalidate)}>🔕 Unfollow</Button>
              ) : (
                <Button onClick={() => follow(person.uuid).then(revalidate)}>🔔 Follow</Button>
              ))}
          </Group>
          <Text c="dimmed" size="xl">
            Joined {formatTimeStamp(person.created_at)}
          </Text>
        </Stack>
      </Group>
      <Group justify="space-between" px="xl" wrap="nowrap">
        <Stat label="cards rated" value={Number(person.cards_rated)} />
        <Stack>
          <Title size="2rem">
            <PointsNumberFormatter points={person.personal_points_allocated} suffix=" ppts" />
          </Title>
          <Text textWrap="nowrap">total points allocated</Text>
        </Stack>
        <Stat label="tracked decks" value={Number(person.tracked_decks)} />
        <Stat label="followers" value={Number(person.followers)} />
        <Stat label="following" value={Number(person.following)} />
      </Group>
    </Stack>
  )
}
