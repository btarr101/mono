import {
  Button,
  Center,
  Group,
  Modal,
  type ModalProps,
  Paper,
  Stack,
  Text,
  Textarea,
} from '@mantine/core'
import { useClipboard, useDisclosure } from '@mantine/hooks'
import { notifications } from '@mantine/notifications'
import { FilesIcon, GlobeIcon } from '@phosphor-icons/react'
import { useState } from 'react'
import { match } from 'ts-pattern'

import type { Deck } from '../../types/bindings/Deck'

export type DeckSource =
  | {
      ty: 'url'
      url: string
    }
  | {
      ty: 'decklist'
      deck: Deck
    }

export type DeckSourceButtonProps = {
  source: DeckSource
}

export const DeckSourceButton = ({ source }: DeckSourceButtonProps) => {
  const [opened, { open, close }] = useDisclosure(false)

  const label = match(source.ty)
    .with('url', () => (
      <Group wrap="nowrap">
        <GlobeIcon size={32} />
        <Text textWrap="nowrap">Source: URL</Text>
      </Group>
    ))
    .with('decklist', () => (
      <Group wrap="nowrap">
        <FilesIcon size={32} />
        <Text textWrap="nowrap">Source: Decklist</Text>
      </Group>
    ))
    .exhaustive()

  return (
    <>
      <Paper withBorder flex={1} h={'100%'}>
        <Center h="100%" p="sm">
          <Stack>
            {label}
            <Button onClick={open}>View</Button>
          </Stack>
        </Center>
      </Paper>
      <ViewDeckSourceModal opened={opened} source={source} title={label} onClose={close} />
    </>
  )
}

export type ViewDecklistSourceModalProps = ModalProps & {
  source: DeckSource
}

export const ViewDeckSourceModal = ({ source, ...modalProps }: ViewDecklistSourceModalProps) => {
  const clipboard = useClipboard()

  const onCopy = (content: string) => {
    clipboard.copy(content)
    notifications.show({
      title: 'Copied to clipboard',
      message: null,
      autoClose: 1000,
    })
  }

  return (
    <Modal
      centered
      miw={'fit-content'}
      styles={{
        content: {
          minWidth: 'fit-content',
        },
      }}
      {...modalProps}
    >
      {match(source)
        .with({ ty: 'url' }, () => <></>)
        .with({ ty: 'decklist' }, ({ deck: { commanders, maindeck } }) => {
          const commanderLines = commanders.map(({ name }) => `1 ${name}`)
          const mainDeckLines = maindeck.map(({ count, card: { name } }) => `${count} ${name}`)
          const content = [...commanderLines, ...mainDeckLines].join('\n')

          return <ViewDeckSourceModalDecklistContent decklist={content} onCopy={onCopy} />
        })
        .exhaustive()}
    </Modal>
  )
}

type ViewDeckSourceModalDecklistContentProps = {
  decklist: string
  onCopy: (content: string) => void
}

const ViewDeckSourceModalDecklistContent = ({
  decklist,
  onCopy,
}: ViewDeckSourceModalDecklistContentProps) => {
  const [editedContent, setEditedContent] = useState(decklist)

  const edited = editedContent !== decklist

  return (
    <Group align="start">
      <Stack flex={1} gap="xs">
        <Textarea
          mih={360}
          styles={{
            root: { display: 'flex', flexDirection: 'column', flexGrow: 1, minHeight: 0 },
            wrapper: {
              display: 'flex',
              flexDirection: 'column',
              flexGrow: 1,
              minHeight: 0,
            },
            input: { flexGrow: 1, height: '100%' },
          }}
          value={editedContent}
          onChange={event => setEditedContent(event.target.value)}
        />
        <Text c="dimmed" size="xs">
          {edited && '* edited'}
        </Text>
      </Stack>

      <Button onClick={() => onCopy(editedContent)}>Copy</Button>
    </Group>
  )
}
