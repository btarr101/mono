import { useEffect } from 'react'

import { useApi } from '../contexts/ApiContext'
import { useEasel, useEaselLedColorUpdated } from '../contexts/EaselContext'

export const LedSyncher = () => {
  const { latestLeds, sendLedUpdate } = useApi()
  const { updateLeds: setLedsByLatest } = useEasel()

  // Downsync
  useEffect(() => {
    if (!latestLeds) return

    setLedsByLatest(latestLeds)
  }, [latestLeds, setLedsByLatest])

  // Upsync
  useEaselLedColorUpdated(sendLedUpdate)

  return null
}
