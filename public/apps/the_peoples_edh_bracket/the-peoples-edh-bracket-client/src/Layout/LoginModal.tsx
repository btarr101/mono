import { Center, Divider, Modal, type ModalProps, Stack, Text, Title } from '@mantine/core'
import { GoogleLogin } from '@react-oauth/google'

import { useLogin } from '../hooks/useAuth'

export const LoginModal = (props: ModalProps) => {
  const login = useLogin()

  return (
    <Modal {...props} centered>
      <Center p="lg">
        <Stack>
          <Title order={1} ta={'center'} textWrap="nowrap">
            Be a part of the{' '}
            <Text inherit c="var(--mantine-primary-color-filled)" component="span">
              bracket
            </Text>{' '}
          </Title>
          <Text>
            Log in to rate cards, track analyzed decks, and work for the good of the people who play
            EDH.
          </Text>
          <Divider />
          <GoogleLogin
            useOneTap
            onError={() => console.error('Login Failed')}
            onSuccess={({ credential }) => {
              if (!credential) return
              login({
                ty: 'google',
                jwt: credential,
              })
              props.onClose()
            }}
          />
        </Stack>
      </Center>
    </Modal>
  )
}
