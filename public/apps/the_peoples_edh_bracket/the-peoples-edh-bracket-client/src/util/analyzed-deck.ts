import type { AnalyzedDeck } from '../types/bindings/AnalyzedDeck'

const NEW_ANALYZED_DECK_KEY = 'new-analyzed-deck'

export const readNewAnalyzedDeck = () => {
  const rawValue = sessionStorage.getItem(NEW_ANALYZED_DECK_KEY)
  if (!rawValue) return null

  try {
    return JSON.parse(rawValue) as AnalyzedDeck
  } catch {
    return null
  }
}

export const setNewAnalyzedDeck = (newAnalyzedDeck: AnalyzedDeck | null) => {
  if (newAnalyzedDeck) {
    sessionStorage.setItem(NEW_ANALYZED_DECK_KEY, JSON.stringify(newAnalyzedDeck))
  } else {
    sessionStorage.removeItem(NEW_ANALYZED_DECK_KEY)
  }
}
