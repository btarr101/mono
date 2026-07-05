import { Button, Group, Input, Stack, Text, Textarea, Title } from '@mantine/core'
import { useForm } from '@mantine/form'
import { notifications } from '@mantine/notifications'
import { useMutation } from '@tanstack/react-query'
import { useState } from 'react'
import { useNavigate } from 'react-router'

import { postAnalyze } from '../../api/decks'
import type { DecklistMaindeckEntry } from '../../types/bindings/DecklistMaindeckEntry'
import type { PostAnalyzeBody } from '../../types/bindings/PostAnalyzeBody'
import { setNewAnalyzedDeck } from '../../util/analyzed-deck'
import { DecklistFormModal } from './DecklistFormModal'
import { parseDecklist } from './parse-decklist'

export const AnalyzePage = () => {
  const navigate = useNavigate()

  const { mutateAsync } = useMutation({
    mutationFn: postAnalyze,
    onSuccess: analyzeDeckResponse => {
      if (analyzeDeckResponse.type === 'invalid') {
        const invalidCards = [
          ...analyzeDeckResponse.invalid_commanders,
          ...analyzeDeckResponse.invalid_maindeck,
        ]

        notifications.show({
          title: 'Failed to analyze deck - the following cards could not be found by name:',
          message: invalidCards.join(', '),
          color: 'red',
          autoClose: false,
        })

        return
      }

      setNewAnalyzedDeck(analyzeDeckResponse)
      navigate('/analyze/new')
    },
  })

  return (
    <Stack justify="stretch" mih="100dvh" p="xl" w="100%">
      <Hero />
      <MoxfieldUrlForm />
      <DecklistForm onAnalyze={mutateAsync} />
    </Stack>
  )
}

const Hero = () => (
  <Stack gap="lg">
    <Title size="4rem" textWrap="nowrap">
      Analyze a{' '}
      <Text inherit c="var(--mantine-primary-color-filled)" component="span">
        Deck.
      </Text>
    </Title>
    <Text c="dimmed" maw={540} size="xl">
      Enter a moxfield URL or paste your decklist to see it{"'"}s community driven power level.
    </Text>
  </Stack>
)

export type AnalyzeFormProps = {
  onAnalyze: (body: PostAnalyzeBody) => Promise<unknown>
}

const MoxfieldUrlForm = () => {
  return (
    <form>
      <Stack flex={1} gap="md">
        <Title order={1}>Option 1: Moxfield URL (🚧 Under construction 🚧)</Title>
        <Group>
          <Input disabled placeholder="https://moxfield.com/decks/..." w="50%" />
          <Button disabled w={'fit-content'}>
            Analyze
          </Button>
        </Group>
      </Stack>
    </form>
  )
}

const DECKLIST_PLACEHOLDER = `1 Aftermath Analyst
1 Amulet of Vigor
1 Archaeological Dig
1 Archdruid's Charm
1 Arid Archway
1 Ashaya, Soul of the Wild
...`

const DecklistForm = ({ onAnalyze }: AnalyzeFormProps) => {
  const form = useForm({
    mode: 'controlled',
    initialValues: {
      decklist: '',
    },
    validate: {
      decklist: decklist => {
        const result = parseDecklist(decklist)
        if (result.ty === 'error') {
          return result.error.join('\n')
        }

        return null
      },
    },
  })

  const [validDecklist, setValidDecklist] = useState<DecklistMaindeckEntry[] | null>(null)

  return (
    <>
      <form
        style={{
          flex: 1,
          flexDirection: 'column',
          display: 'flex',
          height: '100%',
        }}
        onSubmit={form.onSubmit(({ decklist }) => {
          const result = parseDecklist(decklist)

          // This should never come up due to validation
          if (result.ty === 'error') throw new Error('Unable to parse decklist')

          setValidDecklist(result.value)
        })}
      >
        <Stack flex={1} gap="md">
          <Title order={1}>Option 2: Decklist</Title>
          <Textarea
            key={form.key('decklist')}
            {...form.getInputProps('decklist')}
            placeholder={DECKLIST_PLACEHOLDER}
            styles={{
              root: { display: 'flex', flexDirection: 'column', flexGrow: 1, minHeight: 0 },
              wrapper: { display: 'flex', flexDirection: 'column', flexGrow: 1, minHeight: 0 },
              input: { flexGrow: 1, height: '100%' },
            }}
          />
          <Button type="submit" w={'fit-content'}>
            Analyze
          </Button>
        </Stack>
      </form>
      {validDecklist && (
        <DecklistFormModal
          decklist={validDecklist}
          opened={validDecklist !== null}
          onAnalyze={onAnalyze}
          onClose={() => setValidDecklist(null)}
        />
      )}
    </>
  )
}
