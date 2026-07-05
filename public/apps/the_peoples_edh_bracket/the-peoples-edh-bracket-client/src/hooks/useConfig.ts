import { useQuery } from '@tanstack/react-query'

import { getConfig } from '../api/config'

export const useConfig = () =>
  useQuery({
    queryKey: ['config'],
    queryFn: getConfig,
  })
