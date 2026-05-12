export type Color = {
  red: number
  green: number
  blue: number
}

export type LedLastUpdated = {
  source: 'client' | 'server'
  timestamp: Date
}

export type Led = {
  color: Color
  lastUpdated?: LedLastUpdated
}
