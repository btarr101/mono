import type { GetPersonsParams } from '../types/bindings/GetPersonsParams'
import type { Person } from '../types/bindings/Person'
import type { PersonEnriched } from '../types/bindings/PersonEnriched'
import type { PersonWithTotalPoints } from '../types/bindings/PersonWithTotalPoints'
import { api } from '.'

export const getPersons = (params: GetPersonsParams) => {
  const searchParams = new URLSearchParams()
  Object.entries(params).forEach(
    ([key, value]) => value !== null && searchParams.append(key, String(value)),
  )

  return api.get('persons', { searchParams }).json<PersonEnriched[]>()
}

export const debugPostPerson = () => {
  return api.post('persons').json<Person>()
}

export const getMe = () => {
  return api.get('persons/me').json<PersonWithTotalPoints>()
}

export const getPerson = (uuid: string) => {
  return api.get(`persons/${uuid}`).json<PersonEnriched>()
}

export const postFollowPerson = (uuid: string) => {
  return api.post(`persons/${uuid}/follow`)
}

export const postUnfollowPerson = (uuid: string) => {
  return api.post(`persons/${uuid}/unfollow`)
}
