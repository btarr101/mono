import type { Position } from '../../types'

export const getPositionsInStroke = (
  positions: Position[],
  strokeStart: Position,
  strokeEnd: Position,
  brushWidth = 52,
) =>
  positions.flatMap((ledPosition, index) => {
    const distance = distanceFromPointToLineSegment(ledPosition, strokeStart, strokeEnd)
    return distance <= brushWidth ? [index] : []
  })

export const distanceFromPointToLineSegment = (
  point: Position,
  segmentStart: Position,
  segmentEnd: Position,
) => {
  const dx = segmentEnd.x - segmentStart.x
  const dy = segmentEnd.y - segmentStart.y

  // 0 length guard
  const lengthSq = dx * dx + dy * dy
  if (lengthSq === 0) {
    return Math.hypot(point.x - segmentStart.x, point.y - segmentStart.y)
  }

  const t = Math.max(
    0,
    Math.min(
      1,
      ((point.x - segmentStart.x) * dx + (point.y - segmentStart.y) * dy) / (dx * dx + dy * dy),
    ),
  )
  const projX = segmentStart.x + t * dx
  const projY = segmentStart.y + t * dy
  return Math.sqrt((point.x - projX) ** 2 + (point.y - projY) ** 2)
}

export const divideVectors = (a: Position, b: Position) => ({
  x: a.x / b.x,
  y: a.y / b.y,
})
