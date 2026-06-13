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
import { useForm } from '@mantine/form'
import { FireIcon, MagnifyingGlassIcon } from '@phosphor-icons/react'
import { Link, useLoaderData, useNavigate } from 'react-router'

import { MtgCardButton, MtgCardButtonGhost } from '../components/MtgCardButton'
import { useDebouncedSearchCards, useGetCards } from '../hooks/useCards'
import type { HomeMetrics } from '../types/bindings/HomeMetrics'

export const HomePage = () => (
  <Stack justify="stretch" mih="100dvh" p="xl" w="100%">
    <Hero />
    <Divider />
    <Trending />
  </Stack>
)

const Hero = () => {
  const navigate = useNavigate()
  const homeMetrics = useLoaderData<HomeMetrics>()

  const form = useForm({
    mode: 'controlled',
    initialValues: {
      q: '',
    },
  })
  const [usedSearchCards, { isDebouncing }] = useDebouncedSearchCards(form.getValues().q || null)

  const isAutocompleteLoading = isDebouncing || usedSearchCards.isFetching

  return (
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
      <form
        onSubmit={form.onSubmit(({ q }) => {
          if (q) navigate(`/browse?q=${q}`)
        })}
      >
        <Autocomplete
          data={
            isAutocompleteLoading
              ? [
                  {
                    value: '...',
                    disabled: true,
                  },
                ]
              : (usedSearchCards.data?.pages.flat().map(({ name }) => name) ?? [])
          }
          filter={({ options }) => options}
          key={form.key('q')}
          loading={usedSearchCards.isFetching}
          {...form.getInputProps('q')}
          placeholder="Search for a card..."
          rightSection={<MagnifyingGlassIcon />}
          onOptionSubmit={q => {
            navigate(`/browse?q=${q}`)
          }}
        />
      </form>
      <Group justify="space-evenly" wrap="nowrap">
        <Stat label="cards rated" value={homeMetrics.total_cards_rated} />
        <Divider orientation="vertical" />
        <Stat label="people" value={homeMetrics.total_persons} />
        <Divider orientation="vertical" />
        <Stat label="total ratings" value={homeMetrics.total_ratings} />
      </Group>
    </Stack>
  )
}

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
        <TrendingCards />
      </Group>
    </ScrollArea>
  </Stack>
)

const TrendingCards = () => {
  const cards = useGetCards({ q: null, sort: 'trending', page_size: 5 })

  return cards.data
    ? cards.data.pages.flat().map(card => <MtgCardButton card={card} key={card.oracle_id} />)
    : Array.from({ length: 5 }).map((_, index) => <MtgCardButtonGhost key={index} />)
}
