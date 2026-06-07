import ky from 'ky'

import { type AuthState } from '../hooks/useAuth'

export const API_BASE_URL = import.meta.env.VITE_API_BASE_URL ?? '/api'

export const api = ky.create({
  baseUrl: import.meta.env.VITE_API_BASE_URL ?? '/api',
  hooks: {
    beforeRequest: [
      ({ request }) => {
        const authState: AuthState = JSON.parse(localStorage.getItem('auth') ?? 'null') ?? {
          ty: null,
        }

        if (authState.ty === 'debug') {
          request.headers.set('Authorization', `Debug ${authState.personUUID}`)
        }
      },
    ],
  },
})
