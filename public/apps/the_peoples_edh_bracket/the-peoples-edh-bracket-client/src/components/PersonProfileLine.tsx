import { ActionIcon, Avatar, Group, Menu, Text } from '@mantine/core'
import { CaretUpIcon } from '@phosphor-icons/react'
import type { PropsWithChildren } from 'react'

import type { Person } from '../types/bindings/Person'

export type PersonProfileLineProps = PropsWithChildren<{
  loading: boolean
  person?: Person | null
}>

export const PersonProfileLine = ({ loading, person, children }: PersonProfileLineProps) => {
  const anonymous = !loading && !person

  return (
    <Group gap="sm" style={{ minWidth: 0 }} wrap="nowrap">
      <Avatar imageProps={{ referrerPolicy: 'no-referrer' }} size="md" src={person?.picture_url}>
        {anonymous ? '🫏' : ''}
      </Avatar>
      <Group gap={0} style={{ minWidth: 0, flex: 1 }} wrap="nowrap">
        <Text
          size="md"
          style={{
            minWidth: 0,
            flex: 1,
            textOverflow: 'ellipsis',
            overflow: 'hidden',
            whiteSpace: 'nowrap',
          }}
          textWrap="nowrap"
        >
          {loading ? '...' : (person?.username ?? 'Anonymous Donkey')}
        </Text>
        <Menu closeOnItemClick={true} position="top-end">
          <Menu.Target>
            <ActionIcon variant="transparent">
              <CaretUpIcon />
            </ActionIcon>
          </Menu.Target>
          <Menu.Dropdown>{children}</Menu.Dropdown>
        </Menu>
      </Group>
    </Group>
  )
}
