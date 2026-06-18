import { Button, Group, Input, Stack, Text, Textarea, Title } from '@mantine/core'
import { useForm } from '@mantine/form'
import { DecklistFormModal } from './DecklistFormModal'
import { useState } from 'react'
import { useMutation } from '@tanstack/react-query'
import { postAnalyze } from '../../api/decks'
import type { PostAnalyzeBody } from '../../types/bindings/PostAnalyzeBody'
import { parseDecklist } from './parse-decklist'
import type { DecklistMaindeckEntry } from '../../types/bindings/DecklistMaindeckEntry'
import { useNavigate } from 'react-router'
import { notifications } from '@mantine/notifications'
import { setNewAnalyzedDeck } from '../AnalyzedDeckPage/analyzed-deck'

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
  onAnalyze: (body: PostAnalyzeBody) => Promise<any>
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
          return 'Unable to parse decklist'
        }

        return null
      },
    },
  })

  const [validDecklist, setValidDecklist] = useState<DecklistMaindeckEntry[] | null>(null)

  return (
    <>
      <form
        onSubmit={form.onSubmit(({ decklist }) => {
          const result = parseDecklist(decklist)

          // This should never come up due to validation
          if (result.ty === 'error') throw new Error('Unable to parse decklist')

          setValidDecklist(result.value)
        })}
        style={{
          flex: 1,
          flexDirection: 'column',
          display: 'flex',
          height: '100%',
        }}
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
          <Button w={'fit-content'} type="submit">
            Analyze
          </Button>
        </Stack>
      </form>
      {validDecklist && (
        <DecklistFormModal
          onAnalyze={onAnalyze}
          opened={validDecklist !== null}
          onClose={() => setValidDecklist(null)}
          decklist={validDecklist}
        />
      )}
    </>
  )
}
