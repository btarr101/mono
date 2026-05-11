import { useEffect } from 'react'

import { useApi } from '../contexts/ApiContext'
import { useEasel, useEaselLedColorUpdated } from '../contexts/EaselContext'

export const LedSyncher = () => {
  const { latestLeds, sendLedUpdate } = useApi()
  const { setLeds } = useEasel()

  // Downsync
  useEffect(() => {
    if (!latestLeds) return

    setLeds(latestLeds.map(led => led.color))
  }, [latestLeds, setLeds])

  // Upsync
  useEaselLedColorUpdated(sendLedUpdate)

  return null
}
