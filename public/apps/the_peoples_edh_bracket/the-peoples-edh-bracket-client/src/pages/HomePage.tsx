import {
  Autocomplete,
  Button,
  Divider,
  Group,
  NumberFormatter,
  ScrollArea,
  Stack,
  Text,
  Title,
} from '@mantine/core'
import { FireIcon, MagnifyingGlassIcon } from '@phosphor-icons/react'
import { Link } from 'react-router'

import { MtgCardButton } from '../components/MtgCardButton'

const TEMP_CARDS = ['Black Lotus', 'Storm Crow', 'Eddymurk Crab']

export const HomePage = () => (
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
    <Group justify="space-between">
      <Group c="orange" gap="xs">
        <FireIcon size={32} />
        <Title size="2rem">Trending</Title>
      </Group>
      <Button component={Link} to={{ pathname: 'browse', search: '?sort=trending' }}>
        View All
      </Button>
    </Group>

    <ScrollArea mih={'fit-content'} scrollbars="x" style={{ flex: 1 }} w={'100%'}>
      <Group
        h={'100%'}
        justify={'space-evenly'}
        mih={'fit-content'}
        style={{ flex: 1, minHeight: 0 }}
        w={'100%'}
        wrap="nowrap"
      >
        <MtgCardButton />
        <MtgCardButton />
        <MtgCardButton />
        <MtgCardButton />
        <MtgCardButton />
      </Group>
    </ScrollArea>
  </Stack>
)
