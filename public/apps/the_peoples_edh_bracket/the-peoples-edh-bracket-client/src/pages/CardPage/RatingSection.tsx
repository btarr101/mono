import { Alert, Box, Group, Select, Stack, Title } from '@mantine/core'
import { useClipboard } from '@mantine/hooks'
import { notifications } from '@mantine/notifications'
import { InfoIcon } from '@phosphor-icons/react'
import { useWindowVirtualizer } from '@tanstack/react-virtual'
import { parseAsStringLiteral, useQueryState } from 'nuqs'

import { EmptyPlaceholder } from '../../components/EmptyPlaceholder'
import { Rating, RatingGhost } from '../../components/Rating'
import { useReactVirtualScrollRestoration } from '../../hooks/react-virtual-ext'
import { useMe } from '../../hooks/usePersons'
import { useGetRatings, usePersonRating, usePutRating, useRating } from '../../hooks/useRatings'
import { RatingInput } from './RatingInput'

export type RatingSectionProps = {
  cardOracleId: string
}

export const RatingSection = ({ cardOracleId }: RatingSectionProps) => {
  const clipboard = useClipboard()
  const [sort, setSort] = useQueryState(
    'sort',
    parseAsStringLiteral([
      'liked',
      'disliked',
      'controversial',
      'recent',
      'highest_rated',
      'lowest_rated',
    ] as const).withDefault('liked'),
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
  const usedRatings = useGetRatings({
    card_oracle_id: cardOracleId,
    rater_person_uuid: null,
    q: null,
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

  const hasNotRated = !loggedInPersonUUID || !usedLoggedInPersonRating.data
  const showEndMessage =
    !usedRatings.hasNextPage &&
    (usedRatings.data?.pages.filter(page => page.length > 0).length ?? 0) > 1

  const ratingsCount = useRatingsPages?.length ?? 0
  const virtualizer = useWindowVirtualizer({
    count: ratingsCount,
    estimateSize: () => 150,
    overscan: 3,
  })

  const virtualItems = virtualizer.getVirtualItems()
  const first = virtualItems.at(0)?.start ?? 0
  const end = virtualItems.length ? virtualizer.getTotalSize() - (virtualItems.at(-1)?.end ?? 0) : 0

  useReactVirtualScrollRestoration(virtualizer)

  // Infinite scrolling
  // useLayoutEffect(() => {
  //   if (end === 0 && usedRatings.hasNextPage && !usedRatings.isFetching) {
  //     usedRatings.fetchNextPage()
  //   }
  // }, [usedRatings, end])

  return (
    <Stack>
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
              {
                value: 'highest_rated',
                label: '👑 Highest Rated',
              },
              {
                value: 'lowest_rated',
                label: '🗑️ Lowest Rated',
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
            <Box>
              <Box h={first} />
              {virtualItems.map(item => {
                const rating = useRatingsPages[item.index]
                if (!rating) return null

                return (
                  <Box key={rating.uuid} py="sm">
                    <Rating
                      rating={rating}
                      onPin={() => setPinnedRatingUUID(rating.uuid)}
                      onShare={() => onShare(rating.uuid)}
                    />
                  </Box>
                )
              })}
              <Box h={end} />
            </Box>
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
    </Stack>
  )
}
