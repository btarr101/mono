import 'react-virtualized/styles.css'

import { Alert, Box, Group, Select, Stack, Title } from '@mantine/core'
import { useClipboard } from '@mantine/hooks'
import { notifications } from '@mantine/notifications'
import { InfoIcon } from '@phosphor-icons/react'
import { parseAsStringLiteral, useQueryState } from 'nuqs'
import { useRef } from 'react'
import { AutoSizer, CellMeasurer, CellMeasurerCache, List, WindowScroller } from 'react-virtualized'

import { EmptyPlaceholder } from '../../components/EmptyPlaceholder'
import { Rating, RatingGhost } from '../../components/Rating'
import { useMe } from '../../hooks/usePersons'
import { usePersonRating, usePutRating, useRating, useRatings } from '../../hooks/useRatings'
import { RatingInput } from './RatingInput'

export type RatingSectionProps = {
  cardOracleId: string
}

export const RatingSection = ({ cardOracleId }: RatingSectionProps) => {
  const clipboard = useClipboard()

  const [sort, setSort] = useQueryState(
    'sort',
    parseAsStringLiteral(['liked', 'disliked', 'controversial', 'recent'] as const).withDefault(
      'liked',
    ),
  )
  const [pinnedRatingUUID, setPinnedRatingUUID] = useQueryState('pinned', {
    clearOnDefault: true,
  })
  const onShare = (ratingUUID: string) => {
    clipboard.copy(`${window.location}?pinned=${ratingUUID}`)
    notifications.show({
      title: 'Copied share url to clipboard',
      message: null,
      autoClose: 1000,
    })
  }

  const me = useMe()
  const loggedInPersonUUID = me.data?.uuid ?? null
  const usedLoggedInPersonRating = usePersonRating(cardOracleId, loggedInPersonUUID)
  const usedPinnedRating = useRating(pinnedRatingUUID)
  const usedRatings = useRatings({
    card_oracle_id: cardOracleId,
    rater_person_uuid: null,
    sort,
    page_size: 10,
  })
  const usedPutRating = usePutRating()

  const useRatingsPages = usedRatings.data?.pages
    .flat()
    .filter(
      rating => rating.rater_person_uuid !== loggedInPersonUUID && rating.uuid !== pinnedRatingUUID,
    )

  const saveRating = ({ points, reason }: { points: number | null; reason: string | null }) =>
    usedPutRating
      .mutateAsync({
        card_oracle_id: cardOracleId,
        points: (points ?? 0.0).toString(),
        reason: reason || null,
      })
      .then(() => {})

  const cache = useRef(new CellMeasurerCache({ fixedWidth: true, defaultHeight: 150 }))
  const hasNotRated = !loggedInPersonUUID || !usedLoggedInPersonRating.data
  const showEndMessage =
    !usedRatings.hasNextPage &&
    (usedRatings.data?.pages.filter(page => page.length > 0).length ?? 0) > 1

  return (
    <>
      {loggedInPersonUUID && (
        <Stack gap="sm">
          {usedLoggedInPersonRating.isPending ? (
            <RatingGhost />
          ) : (
            <>
              {!usedLoggedInPersonRating.data && (
                <Alert
                  color="orange"
                  icon={<InfoIcon />}
                  title="You haven't rated this card yet."
                  variant="light"
                />
              )}
              <RatingInput
                rating={usedLoggedInPersonRating.data ?? null}
                onSave={saveRating}
                onShare={
                  usedLoggedInPersonRating.data
                    ? () =>
                        usedLoggedInPersonRating.data && onShare(usedLoggedInPersonRating.data.uuid)
                    : undefined
                }
              />
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
            disabled={useRatingsPages?.length === 0}
            value={sort}
            onChange={newSort => setSort(newSort)}
          />
        </Group>
        <Stack gap={0}>
          {pinnedRatingUUID && (
            <Box py="sm">
              {usedPinnedRating.isPending ? (
                <RatingGhost />
              ) : (
                usedPinnedRating.data && (
                  <Rating
                    key={usedPinnedRating.data.uuid}
                    pinned={true}
                    rating={usedPinnedRating.data}
                    onPin={() => setPinnedRatingUUID(null)}
                  />
                )
              )}
            </Box>
          )}

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
            <EmptyPlaceholder
              {...(hasNotRated
                ? {
                    title: '😭 No ratings yet!',
                    subText: 'Ratings will show up down here',
                  }
                : {
                    title: '👀 No other ratings yet',
                    subText: 'You have been the only one to rate this card so far. Stand proud 😤!',
                  })}
            />
          )}
          {showEndMessage && (
            <EmptyPlaceholder
              subText="The journey is complete, you may rest now 🛌."
              title="The end."
            />
          )}
        </Stack>
      </Stack>
    </>
  )
}
