import { Avatar, Button, Group, Stack, Tabs, Text, Title } from '@mantine/core'
import { parseAsStringLiteral, useQueryState } from 'nuqs'
import { useLayoutEffect } from 'react'
import { useLoaderData } from 'react-router'

import { BackAnchor } from '../../components/BackAnchor'
import { Stat } from '../../components/Stat'
import type { Person } from '../../types/bindings/Person'
import { formatTimeStamp } from '../../util'
import { RatingsPanelContent } from './RatingsPanelContent'

const TABS = ['ratings', 'decks', 'followers', 'followees']

export const ProfilePage = () => {
  const { person } = useLoaderData<{ person: Person }>()
  const [tab, setTab] = useQueryState(
    'tab',
    parseAsStringLiteral(TABS).withDefault('ratings').withOptions({
      clearOnDefault: false,
    }),
  )

  // don't carry over scroll
  useLayoutEffect(() => {
    window.scrollTo({ top: 0, left: 0, behavior: 'auto' })
  }, [person.uuid])

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
      </Tabs>
    </Stack>
  )
}

type HeadSectionProps = {
  person: Person
}

export const HeadSection = ({ person }: HeadSectionProps) => (
  <Stack gap="xl">
    <Group align="stretch">
      <Avatar imageProps={{ referrerPolicy: 'no-referrer' }} size="96" src={person.picture_url} />
      <Stack gap="xs" h="100%">
        <Group>
          <Title size="2rem" textWrap="nowrap">
            {person.username}
          </Title>
          {/*for later 🔕*/}
          <Button>🔔 Follow</Button>
        </Group>
        <Text c="dimmed" size="xl">
          Joined {formatTimeStamp(person.created_at)}
        </Text>
      </Stack>
    </Group>
    <Group justify="space-between" px="xl" wrap="nowrap">
      <Stat label="cards rated" value={0} />
      <Stat label="points allocated" suffix=" ppts" value={0} />
      <Stat label="tracked decks" value={0} />
      <Stat label="followers" value={0} />
      <Stat label="following" value={0} />
    </Group>
  </Stack>
)
