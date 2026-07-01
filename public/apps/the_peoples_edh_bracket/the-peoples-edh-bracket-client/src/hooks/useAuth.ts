import { useLocalStorage } from '@mantine/hooks'
import { useQueryClient } from '@tanstack/react-query'
import { useRevalidator } from 'react-router'
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
  const { revalidate } = useRevalidator()

  return (params: LoginParams) => {
    const newAuthState = match(params)
      .returnType<AuthState>()
      .with({ ty: 'debug' }, ({ personUUID }) => ({ ty: 'debug', personUUID }))
      .with({ ty: 'google' }, ({ jwt }) => ({ ty: 'google', jwt }))
      .exhaustive()

    setAuthState(newAuthState)

    queryClient.invalidateQueries()
    revalidate()
  }
}

export const useLogout = () => {
  const queryClient = useQueryClient()
  const [authState, setAuthState] = useAuthState()
  const { revalidate } = useRevalidator()

  return () => {
    if (authState.ty !== null) {
      setAuthState({ ty: null })
    }

    queryClient.clear()
    revalidate()
  }
}
