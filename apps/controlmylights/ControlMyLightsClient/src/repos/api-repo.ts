import { type Options } from 'react-use-websocket'

export type LedDTO = {
  color: {
    red: number
    green: number
    blue: number
  }
  last_updated: Date
}

export const websocketOptions = {
  disableJson: true,
  shouldReconnect: () => true,
  heartbeat: {
    interval: 60000,
    message: 'ping',
    returnMessage: 'pong',
  },
} satisfies Options
