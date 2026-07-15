import { Button, Group, Stack, Text, Title } from '@mantine/core'
import { notifications } from '@mantine/notifications'
import { useMutation } from '@tanstack/react-query'
import { useLoaderData, useNavigate } from 'react-router'

import { deleteTrackedDeck } from '../api/decks'
import { BackAnchor } from '../components/BackAnchor'
import { DeckAnalysis } from '../components/DeckAnalysis'
import { ViewablePersonProfileLine } from '../components/ViewablePersonProfileLine'
import { useMe, usePerson } from '../hooks/usePersons'
import type { TrackedDeckWithAnalysis } from '../types/bindings/TrackedDeckWithAnalysis'
import { formatTimeStamp, safeNavigate } from '../util'

export const TrackedDeckPage = () => {
  const navigate = useNavigate()
  const { trackedDeck } = useLoaderData<{ trackedDeck: TrackedDeckWithAnalysis }>()

  const me = useMe()
  const tracker = usePerson(trackedDeck.tracker_person_uuid)

  const { mutate, isPending } = useMutation({
    mutationFn: deleteTrackedDeck,
  })

  const onDelete = () =>
    mutate(trackedDeck.uuid, {
      onSuccess: () => {
        notifications.show({
          message: `'${trackedDeck.name}' deleted`,
        })
        safeNavigate(navigate, -1, '/analyze')
      },
    })

  return (
    <Stack gap="xl" justify="stretch" mih="100dvh" p="xl" w="100%">
      <BackAnchor fallback="/analyze" />
      <Stack gap="xs">
        <Group align="start" justify="space-between">
          <Title>{trackedDeck.name}</Title>
          {me.data?.uuid === trackedDeck.tracker_person_uuid && (
            <Button color="red" loading={isPending} size="lg" onClick={onDelete}>
              Delete
            </Button>
          )}
        </Group>
        <Group align="center">
          Tracked by <ViewablePersonProfileLine loading={tracker.isPending} person={tracker.data} />
        </Group>
        <Group>
          <Text c="dimmed" size="xs">
            {formatTimeStamp(trackedDeck.created_at)}
          </Text>
          {trackedDeck.url_source && (
            <Text c="dimmed" size="xs">
              (last synced {formatTimeStamp(trackedDeck.last_synced)})
            </Text>
          )}
        </Group>
      </Stack>
      <DeckAnalysis analyzedDeck={trackedDeck} />
    </Stack>
  )
}
