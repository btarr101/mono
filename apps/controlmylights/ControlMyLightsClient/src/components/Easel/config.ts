import { EASEL_IMAGE } from '../../constants'

export const PAD = 32
export const LED_RADIUS = 52
export const LED_HITBOX_RADIUS = 12
export const POSITIONS_AND_COLORS = Array.from({ length: 24 }).flatMap((_, row) =>
  Array.from({ length: 48 }).map((_, col) => ({
    color: {
      red: Math.random() * 255,
      green: Math.random() * 255,
      blue: Math.random() * 255,
    },
    x: PAD + ((EASEL_IMAGE.width - PAD) / 48) * col,
    y: PAD + ((EASEL_IMAGE.height - PAD) / 24) * row,
  })),
)
export const POSITIONS = POSITIONS_AND_COLORS.map(({ x, y }) => ({ x, y }))
