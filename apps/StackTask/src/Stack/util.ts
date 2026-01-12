export type Position = { x: number; y: number }

export const getCenter = (boundingRect: DOMRect) => ({
  x: (boundingRect.left + boundingRect.right) / 2,
  y: (boundingRect.top + boundingRect.bottom) / 2,
})

export const getDistance = (a: Position, b: Position) =>
  Math.sqrt((a.x - b.x) ** 2 + (a.y - b.y) ** 2)
