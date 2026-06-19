import { Autocomplete, Button, Group, Select, Stack, Table } from '@mantine/core'
import { MagnifyingGlassIcon } from '@phosphor-icons/react'
import { useState } from 'react'
import { Link } from 'react-router'

import { LoadingImage } from '../../components/LoadingImage'
import { PointsNumberFormatter } from '../../components/PointsNumberFormatter'
import type { CardWithGlobalPoints } from '../../types/bindings/CardWithGlobalPoints'
import type { Deck } from '../../types/bindings/Deck'
import type { DeckMaindeckEntry } from '../../types/bindings/DeckMaindeckEntry'

export type DeckTableProps = {
  deck: Deck
}

type SortOption = 'highest_rated' | 'lowest_rated'
type CardEntry =
  | {
      ty: 'commander'
      card: CardWithGlobalPoints
    }
  | ({
      ty: 'maindeck-entry'
    } & DeckMaindeckEntry)

export const DeckTable = ({ deck }: DeckTableProps) => {
  const [q, setQ] = useState<string | null>(null)
  const [sort, setSort] = useState<SortOption | null>(null)

  const commanderEntries = deck.commanders.map(
    commander =>
      ({
        ty: 'commander',
        card: commander,
      }) satisfies CardEntry,
  )
  const maindeckEntries = deck.maindeck.map(
    entry => ({ ty: 'maindeck-entry', ...entry }) satisfies CardEntry,
  )

  const sortCompareFunc = (a: CardEntry, b: CardEntry) => {
    if (sort !== undefined) {
      const difference = parseFloat(a.card.global_points) - parseFloat(b.card.global_points)
      if (difference < 0) return Number(sort === 'highest_rated') * 2 - 1
      if (difference > 0) return 1 - Number(sort === 'highest_rated') * 2
    }

    return a.card.name.localeCompare(b.card.name)
  }

  const sortedCardEntries = [
    ...commanderEntries.toSorted(sortCompareFunc),
    ...maindeckEntries.toSorted(sortCompareFunc),
  ]

  const cardNames = sortedCardEntries.map(({ card: { name } }) => name)
  const filteredEntries = sortedCardEntries.filter(entry =>
    entry.card.name.toLowerCase().startsWith(q?.toLowerCase() ?? ''),
  )

  return (
    <Stack>
      <Group w={'100%'}>
        <Autocomplete
          data={cardNames}
          placeholder="Search for a card..."
          rightSection={<MagnifyingGlassIcon />}
          style={{ flex: 1 }}
          value={q ?? ''}
          onChange={newValue => setQ(newValue ?? undefined)}
        />
        <Select
          data={[
            { value: 'highest_rated', label: '👑 Highest Rated' },
            { value: 'lowest_rated', label: '🗑️ Lowest Rated' },
          ]}
          placeholder="sort by"
          value={sort}
          onChange={newSort => setSort(newSort)}
        />
      </Group>
      <Table verticalSpacing={4}>
        <colgroup>
          <col />
          <col />
          <col />
          <col style={{ width: '100%' }} />
        </colgroup>
        <Table.Thead>
          <Table.Tr>
            <Table.Th>Count</Table.Th>
            <Table.Th>Card</Table.Th>
            <Table.Th>pts</Table.Th>
            <Table.Th ta="right" />
          </Table.Tr>
        </Table.Thead>
        <Table.Tbody>
          {filteredEntries.map(entry => (
            <Table.Tr key={entry.card.oracle_id}>
              <Table.Td>{entry.ty === 'commander' ? 1 : entry.count}</Table.Td>
              <Table.Td
                style={{
                  whiteSpace: 'nowrap',
                }}
              >
                <Group wrap="nowrap">
                  <LoadingImage src={entry.card.image_uri} w={32} />
                  {entry.ty === 'commander' && '👑 '}
                  {entry.card.name}
                </Group>
              </Table.Td>
              <Table.Td
                style={{
                  textWrap: 'nowrap',
                }}
              >
                <PointsNumberFormatter points={entry.card.global_points} suffix=" pts" />
              </Table.Td>
              <Table.Td ta="right">
                <Button component={Link} to={{ pathname: `/browse/${entry.card.oracle_id}` }}>
                  View
                </Button>
              </Table.Td>
            </Table.Tr>
          ))}
        </Table.Tbody>
      </Table>
    </Stack>
  )
}
