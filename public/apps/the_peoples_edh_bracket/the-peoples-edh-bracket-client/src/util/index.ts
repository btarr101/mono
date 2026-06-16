import type { NavigateFunction } from 'react-router'

export const safeNavigate = (navigate: NavigateFunction, delta: number, fallback: string) => {
  const currentIdx: number = window.history.state?.idx ?? 0
  const targetIdx = currentIdx + delta

  if (targetIdx >= 0) {
    navigate(delta)
  } else {
    navigate(fallback)
  }
}

const timeZone = Intl.DateTimeFormat().resolvedOptions().timeZone
const formatter = new Intl.DateTimeFormat('en-US', {
  timeZone,
  dateStyle: 'short',
  timeStyle: 'short',
})

export const formatTimeStamp = (timestamp: string) => {
  try {
    return formatter.format(new Date(timestamp))
  } catch {
    return timestamp
  }
}
