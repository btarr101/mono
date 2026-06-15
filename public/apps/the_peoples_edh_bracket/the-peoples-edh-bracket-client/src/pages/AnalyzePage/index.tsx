import { Button, Group, Input, Stack, Text, Textarea, Title } from '@mantine/core'
import { useForm } from '@mantine/form'
import { DecklistFormModal } from './DecklistFormModal'
import { useState } from 'react'
import { useMutation } from '@tanstack/react-query'
import { postAnalyze } from '../../api/decks'
import type { PostAnalyzeBody } from '../../types/bindings/PostAnalyzeBody'

export const AnalyzePage = () => {
  const { mutateAsync } = useMutation({
    mutationFn: postAnalyze,
    onSuccess: response => console.log(response),
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
        <Title order={1}>Option 1: Moxfield URL</Title>
        <Group>
          <Input placeholder="https://moxfield.com/decks/..." w="50%" />
          <Button w={'fit-content'}>Analyze</Button>
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

// God this is so fucking hacky and bad
// SHIT I NEED TO HANDLE MULTIPLE COPIES AHHHHH
export const parseCardNames = (decklist: string) =>
  Array.from(
    new Set(
      decklist
        .split('\n')
        .map(line => line.trim())
        .filter(Boolean)
        .map(line =>
          line
            .replace(/^\d+\s+/, '') // strip leading counts
            .replace(/\s+\(.*$/, '') // strips set info
            .replaceAll(' / ', ' // ') // for double sided cards - they use two //
            .trim(),
        ),
    ),
  )

const DecklistForm = ({ onAnalyze }: AnalyzeFormProps) => {
  const form = useForm({
    mode: 'controlled',
    initialValues: {
      decklist: '',
    },
    validate: {
      decklist: decklist => {
        try {
          console.log(parseCardNames(decklist))
        } catch {
          return 'Unable to parse decklist'
        }

        return null
      },
    },
  })

  const [validDecklist, setValidDecklist] = useState<string[] | null>(null)

  return (
    <>
      <form
        onSubmit={form.onSubmit(({ decklist }) => setValidDecklist(parseCardNames(decklist)))}
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
