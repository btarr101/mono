import { Alert, Group, Select, Stack, Title } from '@mantine/core'
import { InfoIcon } from '@phosphor-icons/react'
import { parseAsStringLiteral, useQueryState } from 'nuqs'

import { EmptyPlaceholder } from '../../components/EmptyPlaceholder'
import { RatingGhost } from '../../components/Rating'
import { Rating } from '../../components/Rating'
import { useLoggedInPersonUUID } from '../../hooks/useAuth'
import { usePersonRating, usePutRating, useRating, useRatings } from '../../hooks/useRatings'
import { RatingInput } from './RatingInput'

export type RatingSectionProps = {
  cardOracleId: string
}

export const RatingSection = ({ cardOracleId }: RatingSectionProps) => {
  const [sort, setSort] = useQueryState(
    'sort',
    parseAsStringLiteral(['liked', 'disliked', 'controversial', 'recent'] as const).withDefault(
      'liked',
    ),
  )
  const [pinnedRatingUUID, setPinnedRatingUUID] = useQueryState('pinned', {
    clearOnDefault: true,
  })

  const loggedInPersonUUID = useLoggedInPersonUUID()
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

  const hasNotRated = !loggedInPersonUUID || !usedLoggedInPersonRating.data

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
              <RatingInput rating={usedLoggedInPersonRating.data ?? null} onSave={saveRating} />
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
        <Stack gap="xl">
          {pinnedRatingUUID &&
            (usedPinnedRating.isPending ? (
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
            ))}

          {useRatingsPages === undefined ? (
            Array.from({ length: 3 }).map((_, index) => <RatingGhost key={index} />)
          ) : useRatingsPages.length > 0 ? (
            useRatingsPages.map(rating => (
              <Rating
                key={rating.uuid}
                rating={rating}
                onPin={() => setPinnedRatingUUID(rating.uuid)}
              />
            ))
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
        </Stack>
      </Stack>
    </>
  )
}
