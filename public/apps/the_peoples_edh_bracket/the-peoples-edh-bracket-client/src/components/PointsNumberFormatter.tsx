import { NumberFormatter } from '@mantine/core'

export type PointsNumberFormatterProps = {
  points: string | number
  suffix: ' pts' | ' ppts'
}

export const PointsNumberFormatter = ({ points, suffix }: PointsNumberFormatterProps) => {
  const parsedPoints = typeof points === 'string' ? parseFloat(points) : points
  const cutOffPoints = Math.trunc(parsedPoints * 100) / 100

  return <NumberFormatter fixedDecimalScale decimalScale={2} suffix={suffix} value={cutOffPoints} />
}
