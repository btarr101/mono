import type { GetPersonsParams } from '../types/bindings/GetPersonsParams'
import type { Person } from '../types/bindings/Person'
import type { PersonEnriched } from '../types/bindings/PersonEnriched'
import type { PersonWithTotalPoints } from '../types/bindings/PersonWithTotalPoints'
import { api, API_BASE_URL } from '.'

export const getPersons = (params: GetPersonsParams) => {
  const uri = new URL(`${API_BASE_URL}/persons`)
  Object.entries(params).forEach(
    ([key, value]) => value !== null && uri.searchParams.append(key, String(value)),
  )

  return api.get(uri).json<PersonEnriched[]>()
}

export const debugPostPerson = () => {
  const uri = new URL(`${API_BASE_URL}/persons`)

  return api.post(uri).json<Person>()
}

export const getMe = () => {
  const uri = new URL(`${API_BASE_URL}/persons/me`)

  return api.get(uri).json<PersonWithTotalPoints>()
}

export const getPerson = (uuid: string) => {
  const uri = new URL(`${API_BASE_URL}/persons/${uuid}`)

  return api.get(uri).json<Person>()
}

export const postFollowPerson = (uuid: string) => {
  const uri = new URL(`${API_BASE_URL}/persons/${uuid}/follow`)

  return api.post(uri)
}

export const postUnfollowPerson = (uuid: string) => {
  const uri = new URL(`${API_BASE_URL}/persons/${uuid}/unfollow`)

  return api.post(uri)
}
