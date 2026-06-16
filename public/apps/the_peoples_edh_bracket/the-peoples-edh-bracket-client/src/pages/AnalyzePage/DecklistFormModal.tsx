import {
  Button,
  Center,
  Divider,
  Modal,
  type ModalProps,
  Select,
  Stack,
  Text,
  Title,
} from '@mantine/core'
import { isNotEmpty, useForm } from '@mantine/form'
import type { AnalyzeFormProps } from '.'
import type { DecklistMaindeckEntry } from '../../types/bindings/DecklistMaindeckEntry'

export type DecklistFormModalProps = ModalProps &
  AnalyzeFormProps & {
    decklist?: DecklistMaindeckEntry[]
  }

export const DecklistFormModal = ({
  onAnalyze,
  decklist,
  ...modalProps
}: DecklistFormModalProps) => {
  const form = useForm({
    mode: 'controlled',
    initialValues: {
      commander: null as string | null,
      partner: null as string | null,
    },
    validate: {
      commander: isNotEmpty(),
    },
  })

  return (
    <Modal {...modalProps} centered>
      <form
        onSubmit={form.onSubmit(async ({ commander, partner }) => {
          const commanders = [commander, partner].flatMap(cardName => (cardName ? [cardName] : []))
          await onAnalyze({
            type: 'decklist',
            commanders,
            maindeck: (decklist ?? []).filter(({ name }) => !commanders.includes(name)),
          })
        })}
      >
        <Center p="lg">
          <Stack>
            <Title order={1} ta={'center'} textWrap="nowrap">
              Almost{' '}
              <Text inherit c="var(--mantine-primary-color-filled)" component="span">
                there
              </Text>{' '}
            </Title>
            <Text>Just select which card(s) from the decklist are your commander(s).</Text>
            <Divider />
            <Select
              key={form.key('commander')}
              {...form.getInputProps('commander')}
              label="Commander"
              data={(decklist ?? [])
                .filter(({ name, count }) => count === 1 && name !== form.getValues().partner)
                .map(({ name }) => name)}
              searchable
            />
            <Select
              key={form.key('partner')}
              {...form.getInputProps('partner')}
              label="Partner"
              data={(decklist ?? [])
                .filter(({ name }) => name !== form.getValues().commander)
                .map(({ name }) => name)}
              searchable
              disabled={form.getValues().commander === null}
              clearable
            />
            <Button mx="auto" w="fit-content" type="submit" loading={form.submitting}>
              Submit
            </Button>
          </Stack>
        </Center>
      </form>
    </Modal>
  )
}
