import ky from 'ky'

import { type AuthState } from '../hooks/useAuth'

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL ?? '/api'

export const api = ky.create({
  prefix: API_BASE_URL,
  hooks: {
    beforeRequest: [
      ({ request }) => {
        const authState: AuthState = JSON.parse(localStorage.getItem('auth') ?? 'null') ?? {
          ty: null,
        }

        if (authState.ty === 'debug') {
          request.headers.set('Authorization', `Debug ${authState.personUUID}`)
        }

        if (authState.ty === 'google') {
          request.headers.set('Authorization', `Bearer ${authState.jwt}`)
        }
      },
    ],
  },
})
