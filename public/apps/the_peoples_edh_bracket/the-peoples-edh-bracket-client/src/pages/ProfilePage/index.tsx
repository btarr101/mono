import {
  Avatar,
  Button,
  Group,
  ScrollArea,
  Scroller,
  Stack,
  Tabs,
  Text,
  Title,
} from '@mantine/core'
import { parseAsStringLiteral, useQueryState } from 'nuqs'
import { useLoaderData, useRevalidator } from 'react-router'

import { BackAnchor } from '../../components/BackAnchor'
import { Stat } from '../../components/Stat'
import { useFollowPerson, useMe, useUnfollowPerson } from '../../hooks/usePersons'
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
          <Scroller>
            <Tabs.Tab value="ratings">
              <Text>Ratings</Text>
            </Tabs.Tab>
            <Tabs.Tab value="decks">
              <Text textWrap="nowrap">Tracked Decks</Text>
            </Tabs.Tab>
            <Tabs.Tab value="followers">
              <Text>Followers</Text>
            </Tabs.Tab>
            <Tabs.Tab value="followees">
              <Text>Followees</Text>
            </Tabs.Tab>
          </Scroller>
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
  const me = useMe()

  const { revalidate } = useRevalidator()
  const { mutateAsync: follow } = useFollowPerson()
  const { mutateAsync: unfollow } = useUnfollowPerson()

  return (
    <Stack gap="xl">
      <Group align="center">
        <Avatar imageProps={{ referrerPolicy: 'no-referrer' }} size="64" src={person.picture_url} />
        <Stack gap="xs" h="100%">
          <Group>
            <Title size="xl" textWrap="nowrap">
              {person.username}
            </Title>
          </Group>
          <Text c="dimmed" size="md">
            Joined {formatTimeStamp(person.created_at)}
          </Text>
        </Stack>
      </Group>
      {!me.isLoading &&
        me.data?.uuid !== person.uuid &&
        person.am_following !== null &&
        (person.am_following ? (
          <Button onClick={() => unfollow(person.uuid).then(revalidate)}>🔕 Unfollow</Button>
        ) : (
          <Button onClick={() => follow(person.uuid).then(revalidate)}>🔔 Follow</Button>
        ))}
      <ScrollArea>
        <Group justify="space-between" px="xl" w={'100%'} wrap="nowrap">
          <Stat label="cards rated" titleSize="lg" value={Number(person.cards_rated)} />
          <Stat label="tracked decks" titleSize="lg" value={Number(person.tracked_decks)} />
          <Stat label="followers" titleSize="lg" value={Number(person.followers)} />
          <Stat label="following" titleSize="lg" value={Number(person.following)} />
        </Group>
      </ScrollArea>
    </Stack>
  )
}
