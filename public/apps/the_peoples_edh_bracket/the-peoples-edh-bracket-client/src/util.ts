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
