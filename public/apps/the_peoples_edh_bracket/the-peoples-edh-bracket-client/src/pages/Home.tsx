import {
  Autocomplete,
  Badge,
  Button,
  Card,
  Divider,
  Group,
  Image,
  NumberFormatter,
  Stack,
  Text,
  Title,
} from '@mantine/core'
import { FireIcon, MagnifyingGlassIcon } from '@phosphor-icons/react'

const TEMP_CARDS = ['Black Lotus', 'Storm Crow', 'Eddymurk Crab']

export const Home = () => (
  <Stack h="100dvh" justify="stretch" p="xl" w="100%">
    <Hero />
    <Divider />
    <Trending />
  </Stack>
)

const Hero = () => (
  <Stack gap="lg">
    <Title size="4rem">
      An EDH bracket system{' '}
      <Text inherit c="var(--mantine-primary-color-filled)">
        driven by the people.
      </Text>
    </Title>
    <Text c="dimmed" maw={540} size="xl">
      Tapping into the power of the community for fair games of Magic: The Gathering™.
    </Text>
    <Autocomplete
      data={TEMP_CARDS}
      placeholder="Search for a card..."
      rightSection={<MagnifyingGlassIcon />}
    />
    <Group justify="space-evenly" wrap="nowrap">
      <Stat label="cards rated" value={5_200_000} />
      <Divider orientation="vertical" />
      <Stat label="people" value={18_300} />
      <Divider orientation="vertical" />
      <Stat label="total ratings" value={10_000_300} />
    </Group>
  </Stack>
)

type StatProps = {
  value: number
  label: string
}

const Stat = ({ value, label }: StatProps) => {
  const scales = [
    {
      min: 1000000,
      suffix: 'm',
    },
    {
      min: 1000,
      suffix: 'k',
    },
  ]

  const scale = scales.find(({ min }) => value > min) ?? { min: 1, suffix: undefined }

  return (
    <Stack>
      <Title size="2rem">
        <NumberFormatter decimalScale={1} suffix={scale.suffix} value={value / scale.min} />
      </Title>
      <Text textWrap="nowrap">{label}</Text>
    </Stack>
  )
}

const Trending = () => (
  <Stack gap="lg" style={{ flex: 1, minHeight: 0 }}>
    <Group c="orange" gap="xs">
      <FireIcon size={32} />
      <Title size="2rem">Trending</Title>
    </Group>
    <Group align="stretch" style={{ flex: 1, minHeight: 0 }} wrap="nowrap">
      <MtgCard />
    </Group>
  </Stack>
)

const MtgCard = () => (
  <Card
    h="100%"
    padding="lg"
    shadow="sm"
    style={{ display: 'flex', flexDirection: 'column' }}
    w="auto"
  >
    <Card.Section style={{ flex: 1, minHeight: 0 }}>
      <Image
        src="https://cards.scryfall.io/large/front/0/3/036ef8c9-72ac-46ce-af07-83b79d736538.jpg?1562730661"
        style={{ aspectRatio: '672 / 936', height: '100%', width: 'auto' }}
      />
    </Card.Section>

    <Group justify="space-between" mb="xs" mt="md">
      <Text fw={500}>Norway Fjord Adventures</Text>
      <Badge color="pink">On Sale</Badge>
    </Group>
  </Card>
)
