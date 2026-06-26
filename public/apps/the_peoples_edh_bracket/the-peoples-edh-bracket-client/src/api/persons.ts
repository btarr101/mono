import type { GetPersonsParams } from '../types/bindings/GetPersonsParams'
import type { Person } from '../types/bindings/Person'
import type { PersonWithMetrics } from '../types/bindings/PersonWithMetrics'
import type { PersonWithTotalPoints } from '../types/bindings/PersonWithTotalPoints'
import { api, API_BASE_URL } from '.'

export const getPersons = async (params: GetPersonsParams) => {
  const uri = new URL(`${API_BASE_URL}/persons`)
  Object.entries(params).forEach(
    ([key, value]) => value !== null && uri.searchParams.append(key, String(value)),
  )

  return api.get(uri).json<PersonWithMetrics[]>()
}

export const debugPostPerson = async () => {
  const uri = new URL(`${API_BASE_URL}/persons`)

  return api.post(uri).json<Person>()
}

export const getMe = async () => {
  const uri = new URL(`${API_BASE_URL}/persons/me`)

  return api.get(uri).json<PersonWithTotalPoints>()
}

export const getPerson = async (uuid: string) => {
  const uri = new URL(`${API_BASE_URL}/persons/${uuid}`)

  return api.get(uri).json<Person>()
}
