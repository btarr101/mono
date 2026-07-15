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
import { ArrowSquareOutIcon, FilesIcon, GlobeIcon } from '@phosphor-icons/react'
import { useState } from 'react'
import { Link } from 'react-router'
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

  const label = (
    <Group gap="xs" wrap="nowrap">
      {match(source.ty)
        .with('url', () => (
          <>
            <GlobeIcon size={32} />
            <Text textWrap="nowrap">Source: URL</Text>
          </>
        ))
        .with('decklist', () => (
          <>
            <FilesIcon size={32} />
            <Text textWrap="nowrap">Source: Decklist</Text>
          </>
        ))
        .exhaustive()}
    </Group>
  )

  return (
    <>
      <Paper withBorder flex={1} h={'100%'}>
        <Center h="100%" p="sm">
          <Stack>
            {label}
            {source.ty === 'decklist' ? (
              <Button onClick={open}>View</Button>
            ) : (
              <Button
                component={Link}
                rightSection={<ArrowSquareOutIcon />}
                target="_blank"
                to={source.url}
                w="100%"
              >
                View
              </Button>
            )}
          </Stack>
        </Center>
      </Paper>
      {source.ty === 'decklist' && (
        <ViewDeckSourceModal deck={source.deck} opened={opened} title={label} onClose={close} />
      )}
    </>
  )
}

export type ViewDecklistSourceModalProps = ModalProps & {
  deck: Deck
}

export const ViewDeckSourceModal = ({
  deck: { commanders, maindeck },
  ...modalProps
}: ViewDecklistSourceModalProps) => {
  const clipboard = useClipboard()

  const onCopy = (content: string) => {
    clipboard.copy(content)
    notifications.show({
      title: 'Copied to clipboard',
      message: null,
      autoClose: 1000,
    })
  }

  const commanderLines = commanders.map(({ name }) => `1 ${name}`)
  const mainDeckLines = maindeck.map(({ count, card: { name } }) => `${count} ${name}`)
  const content = [...commanderLines, ...mainDeckLines].join('\n')

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
      <ViewDeckSourceModalDecklistContent decklist={content} onCopy={onCopy} />
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
