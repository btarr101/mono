import { Button, Group, Stack, TextInput, Tooltip } from '@mantine/core'
import { isNotEmpty, useForm } from '@mantine/form'
import { useMutation } from '@tanstack/react-query'
import { useLoaderData, useNavigate } from 'react-router'

import { postTrackedDeck } from '../api/decks'
import { BackAnchor } from '../components/BackAnchor'
import { DeckAnalysis } from '../components/DeckAnalysis'
import { useMe } from '../hooks/usePersons'
import type { AnalyzedDeck } from '../types/bindings/AnalyzedDeck'
import type { AnalyzedDeckWithSource } from '../types/bindings/AnalyzedDeckWithSource'
import { setNewAnalyzedDeck } from '../util/analyzed-deck'

export const NewAnalyzedDeckPage = () => {
  const navigate = useNavigate()
  const { analyzedDeck } = useLoaderData<{ analyzedDeck: AnalyzedDeckWithSource }>()

  const me = useMe()

  const { mutateAsync } = useMutation({
    mutationFn: postTrackedDeck,
    onSuccess: trackedDeck => {
      setNewAnalyzedDeck(null)
      navigate(`/analyze/${trackedDeck.uuid}`)
    },
  })

  const form = useForm({
    mode: 'controlled',
    initialValues: {
      deckName: buildInitialDeckName(analyzedDeck),
    },
    validate: {
      deckName: isNotEmpty('Deck name cannot be empty'),
    },
  })

  return (
    <Stack gap="xl" justify="stretch" mih="100dvh" p="xl" w="100%">
      <BackAnchor fallback="/analyze" />
      <form
        onSubmit={form.onSubmit(async () => {
          const { deckName } = form.values

          await mutateAsync({
            name: deckName,
            url_source: analyzedDeck.url_source,
            commanders: analyzedDeck.deck.commanders.map(({ oracle_id }) => oracle_id),
            maindeck: analyzedDeck.deck.maindeck.map(({ count, card: { oracle_id } }) => ({
              count,
              oracle_id,
            })),
          })
        })}
      >
        <Group align="start" justify="space-between">
          <TextInput
            flex={1}
            key={form.key('deckName')}
            miw="fit-content"
            placeholder="Enter deck name..."
            size="lg"
            {...form.getInputProps('deckName')}
          />
          {!me.data && <Tooltip label="Must be logged in to save a deck." target="#save-button" />}
          <Button
            disabled={!me.data}
            id="save-button"
            loading={form.submitting}
            size="lg"
            type="submit"
          >
            Save to Tracked Decks
          </Button>
        </Group>
      </form>
      <DeckAnalysis analyzedDeck={analyzedDeck} />
    </Stack>
  )
}

const buildInitialDeckName = (analyzedDeck: AnalyzedDeck): string => {
  const commandersString = analyzedDeck.deck.commanders.map(({ name }) => name).join(' + ')
  return `Unnamed ${commandersString} Deck`
}
