import { Button, Center, Code, Divider, Modal, Stack, Text, Title } from '@mantine/core'

export type AnalyzeErrorModalProps = {
  errorText: string | null
  onClose: () => void
}

export const AnalyzeErrorModal = ({ errorText, onClose }: AnalyzeErrorModalProps) => (
  <Modal opened={errorText !== null} onClose={onClose}>
    <form
      onSubmit={event => {
        event.preventDefault()
        event.stopPropagation()
        onClose()
      }}
    >
      <Center p="lg">
        <Stack>
          <Title order={1}>
            There was an{' '}
            <Text inherit c="red" component="span">
              issue
            </Text>{' '}
            analyzing the deck
          </Title>
          <Text>Tough luck.</Text>
          <Code block>
            {errorText?.includes('404')
              ? // Am I lazy as fuck? Yes. This is terrible but the shits I give missed bus
                'Deck for the moxfield url could not be found.'
              : errorText}
          </Code>
          <Divider />
          <Button mx="auto" type="submit" w="fit-content">
            Okay
          </Button>
        </Stack>
      </Center>
    </form>
  </Modal>
)
