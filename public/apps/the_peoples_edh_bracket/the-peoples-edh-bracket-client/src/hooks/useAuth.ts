import { useLocalStorage } from '@mantine/hooks'
import { useQueryClient } from '@tanstack/react-query'

export type AuthState = { ty: null } | { ty: 'debug'; personUUID: string }

export const useAuthState = () =>
  useLocalStorage<AuthState>({
    key: 'auth',
    defaultValue: { ty: null },
  })

export const useLoggedInPersonUUID = () => {
  const [authState] = useAuthState()

  if (authState.ty === 'debug') {
    return authState.personUUID
  }

  return null
}

export type LoginParams = { ty: 'debug'; personUUID: string }

export const useLogin = () => {
  const queryClient = useQueryClient()
  const [, setAuthState] = useAuthState()

  return (params: LoginParams) => {
    if (params.ty === 'debug') {
      setAuthState({ ty: 'debug', personUUID: params.personUUID })
    }

    queryClient.invalidateQueries()
  }
}

export const useLogout = () => {
  const queryClient = useQueryClient()
  const [authState, setAuthState] = useAuthState()

  return () => {
    if (authState) {
      setAuthState({ ty: null })
    }

    queryClient.invalidateQueries()
  }
}
