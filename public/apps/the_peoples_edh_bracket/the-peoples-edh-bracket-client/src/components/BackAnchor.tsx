import { Anchor } from '@mantine/core'
import { useNavigate } from 'react-router'

import { safeNavigate } from '../util'

export type BackAnchorProps = {
  fallback: string
}

export const BackAnchor = ({ fallback }: BackAnchorProps) => {
  const navigate = useNavigate()

  return <Anchor onClick={() => safeNavigate(navigate, -1, fallback)}>{'<-'} Back</Anchor>
}
