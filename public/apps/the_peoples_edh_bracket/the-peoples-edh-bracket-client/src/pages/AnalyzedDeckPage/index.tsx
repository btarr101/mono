import { Anchor, Box, Button, Center, Group, Input, Stack, TextInput, Title } from '@mantine/core'
import { useNavigate } from 'react-router'
import { safeNavigate } from '../../util'
import { ShieldIcon } from '@phosphor-icons/react'
import { BarChart } from '@mantine/charts'
import TEST_DATA from './test_data.json'

export const AnalyzedDeckPage = () => {
  const navigate = useNavigate()

  return (
    <Stack gap="xl" mih="100dvh" justify="stretch" p="xl" w="100%">
      <Anchor w="fit-content" onClick={() => safeNavigate(navigate, -1, '/analyze')}>
        {'<-'} Back
      </Anchor>
      <Group align="start" justify="space-between">
        <TextInput flex={1} size="lg" placeholder="Untitled Deck" />
        <Button size="lg">Save to tracked Decks</Button>
      </Group>
      <Group wrap="nowrap" align="center">
        <Box w={320} h={320} pos="relative">
          <ShieldIcon size={320} weight="thin" />
          <Center pos="absolute" inset={0}>
            <Title size={48}>100 pts</Title>
          </Center>
        </Box>
        <BarChart
          data={TEST_DATA.sort((a, b) => a.pts - b.pts)}
          series={[{ name: 'pts', color: 'var(--mantine-primary-color-filled)' }]}
          dataKey="card"
          h={320}
          xAxisProps={{
            tick: false,
            interval: undefined, // show every label
            angle: -75, // rotate labels
            textAnchor: 'end',
            // height: 120, // give labels room
          }}
          yAxisProps={{
            unit: ' pts',
          }}
        />
      </Group>
    </Stack>
  )
}
