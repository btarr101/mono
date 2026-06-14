import { Button, Group, Input, Stack, Text, Textarea, Title } from '@mantine/core'

export const AnalyzePage = () => {
  return (
    <Stack justify="stretch" mih="100dvh" p="xl" w="100%">
      <Hero />
      <MoxfieldUrlForm />
      <DecklistForm />
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

const DecklistForm = () => {
  return (
    <form
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
          placeholder={DECKLIST_PLACEHOLDER}
          styles={{
            root: { display: 'flex', flexDirection: 'column', flexGrow: 1, minHeight: 0 },
            wrapper: { display: 'flex', flexDirection: 'column', flexGrow: 1, minHeight: 0 },
            input: { flexGrow: 1, height: '100%' },
          }}
        />
        <Button w={'fit-content'}>Analyze</Button>
      </Stack>
    </form>
  )
}
