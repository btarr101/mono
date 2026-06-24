import 'react-virtualized/styles.css'

import { Avatar, Box, Button, Group, Select, Stack, Tabs, Text, Title } from '@mantine/core'
import { useClipboard } from '@mantine/hooks'
import { notifications } from '@mantine/notifications'
import { parseAsStringLiteral, useQueryState } from 'nuqs'
import { useRef } from 'react'
import { useLoaderData } from 'react-router'
import { AutoSizer, CellMeasurer, CellMeasurerCache, List, WindowScroller } from 'react-virtualized'

import { BackAnchor } from '../components/BackAnchor'
import { EmptyPlaceholder } from '../components/EmptyPlaceholder'
import { Rating, RatingGhost } from '../components/Rating'
import { Stat } from '../components/Stat'
import { useRatings } from '../hooks/useRatings'
import type { Person } from '../types/bindings/Person'
import { formatTimeStamp } from '../util'

const TABS = ['ratings', 'decks', 'followers', 'followees']

export const ProfilePage = () => {
  const { person } = useLoaderData<{ person: Person }>()
  const [tab, setTab] = useQueryState(
    'tab',
    parseAsStringLiteral(TABS).withDefault('ratings').withOptions({
      clearOnDefault: false,
    }),
  )

  return (
    <Stack gap="xl" justify="stretch" mih="100vh" p="xl" w="100%">
      <BackAnchor fallback="/browse" />
      <OverviewSection person={person} />
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

type OverviewSectionProps = {
  person: Person
}

export const OverviewSection = ({ person }: OverviewSectionProps) => (
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

export type RatingsPanelContentProps = {
  personUUID: string
}

export const RatingsPanelContent = ({ personUUID }: RatingsPanelContentProps) => {
  const clipboard = useClipboard()
  const [sort, setSort] = useQueryState(
    'rating-sort',
    parseAsStringLiteral(['liked', 'disliked', 'controversial', 'recent'] as const).withDefault(
      'liked',
    ),
  )

  const onShare = (ratingUUID: string) => {
    clipboard.copy(`${window.location}?pinned=${ratingUUID}`)
    notifications.show({
      title: 'Copied share url to clipboard',
      message: null,
      autoClose: 1000,
    })
  }

  const usedRatings = useRatings({
    rater_person_uuid: personUUID,
    card_oracle_id: null,
    sort,
    page_size: 10,
  })
  const useRatingsPages = usedRatings.data?.pages.flat()

  const cache = useRef(new CellMeasurerCache({ fixedWidth: true, defaultHeight: 150 }))
  const showEndMessage =
    !usedRatings.hasNextPage &&
    (usedRatings.data?.pages.filter(page => page.length > 0).length ?? 0) > 1

  return (
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
          disabled={useRatingsPages?.length === 0}
          value={sort}
          onChange={newSort => setSort(newSort)}
        />
      </Group>
      {useRatingsPages === undefined ? (
        Array.from({ length: 1 }).map((_, index) => <RatingGhost key={index} />)
      ) : useRatingsPages.length > 0 ? (
        <WindowScroller>
          {({ height, isScrolling, onChildScroll, scrollTop, registerChild }) => (
            <AutoSizer disableHeight>
              {({ width }) => (
                <Box ref={registerChild}>
                  <List
                    autoHeight
                    containerStyle={{
                      overflow: 'visible',
                    }}
                    deferredMeasurementCache={cache.current}
                    height={height}
                    isScrolling={isScrolling}
                    overscanRowCount={3}
                    rowCount={useRatingsPages.length}
                    rowHeight={cache.current.rowHeight}
                    rowRenderer={({ index, key, parent, style }) => {
                      const rating = useRatingsPages[index]
                      if (!rating) return

                      return (
                        <CellMeasurer
                          cache={cache.current}
                          columnIndex={0}
                          key={key}
                          parent={parent}
                          rowIndex={index}
                          style={{
                            overflowX: 'hidden',
                          }}
                        >
                          {({ registerChild }) => (
                            <Box
                              py="sm"
                              ref={registerChild}
                              style={{ ...style, overflowX: 'visible' }}
                            >
                              <Rating
                                rating={rating}
                                onPin={() => setPinnedRatingUUID(rating.uuid)}
                                onShare={() => onShare(rating.uuid)}
                              />
                            </Box>
                          )}
                        </CellMeasurer>
                      )
                    }}
                    scrollTop={scrollTop}
                    style={{ overflowY: 'visible', overflowX: 'visible' }}
                    width={width}
                    onRowsRendered={({ stopIndex }) => {
                      if (
                        stopIndex >= useRatingsPages.length - 1 &&
                        usedRatings.hasNextPage &&
                        !usedRatings.isFetching
                      ) {
                        usedRatings.fetchNextPage()
                      }
                    }}
                    onScroll={onChildScroll}
                  />
                </Box>
              )}
            </AutoSizer>
          )}
        </WindowScroller>
      ) : (
        <EmptyPlaceholder subText="Ratings will show up down here" title="😭 No ratings yet!" />
      )}
      {showEndMessage && (
        <EmptyPlaceholder
          subText="The journey is complete, you may rest now 🛌."
          title="The end."
        />
      )}
    </Stack>
  )
}
