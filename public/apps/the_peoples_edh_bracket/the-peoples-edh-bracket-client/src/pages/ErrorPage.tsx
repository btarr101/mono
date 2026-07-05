import { Center, Code, Stack, Text, Title } from '@mantine/core'
import { useRouteError } from 'react-router'

export const ErrorPage = () => {
  const error = useRouteError()

  return (
    <Center flex={1} mih="100dvh">
      <Stack>
        <Title size="4rem">
          An{' '}
          <Text inherit c="red" component="span">
            Error
          </Text>{' '}
          Occurred
        </Title>
        <Text c="dimmed" maw={540} size="xl">
          You must have done something wrong.
        </Text>
        <Code block>{String(error)}</Code>
      </Stack>
    </Center>
  )
}
