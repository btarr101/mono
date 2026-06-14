import { useLocalStorage } from '@mantine/hooks'
import { useQueryClient } from '@tanstack/react-query'
import { match } from 'ts-pattern'

export type AuthState =
  | { ty: null }
  | { ty: 'debug'; personUUID: string }
  | { ty: 'google'; jwt: string }

export const useAuthState = () =>
  useLocalStorage<AuthState>({
    key: 'auth',
    defaultValue: { ty: null },
  })

export type LoginParams = Exclude<AuthState, { ty: null }>

export const useLogin = () => {
  const queryClient = useQueryClient()
  const [, setAuthState] = useAuthState()

  return (params: LoginParams) => {
    const newAuthState = match(params)
      .returnType<AuthState>()
      .with({ ty: 'debug' }, ({ personUUID }) => ({ ty: 'debug', personUUID }))
      .with({ ty: 'google' }, ({ jwt }) => ({ ty: 'google', jwt }))
      .exhaustive()

    setAuthState(newAuthState)

    queryClient.invalidateQueries()
  }
}

export const useLogout = () => {
  const queryClient = useQueryClient()
  const [authState, setAuthState] = useAuthState()

  return () => {
    if (authState.ty !== null) {
      setAuthState({ ty: null })
    }

    queryClient.clear()
  }
}
