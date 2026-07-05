import { NumberFormatter, Stack, Text, Title, type TitleSize } from '@mantine/core'
import type { ReactNode } from 'react'

export type StatProps = {
  titleSize?: TitleSize
  value: number
  label: ReactNode
  suffix?: string
}

export const Stat = ({ value, label, suffix, titleSize }: StatProps) => {
  const scales = [
    {
      min: 1000000,
      suffix: 'm',
    },
    {
      min: 1000,
      suffix: 'k',
    },
  ]

  const scale = scales.find(({ min }) => value > min) ?? { min: 1, suffix: undefined }

  return (
    <Stack>
      <Title size={titleSize ?? '2rem'}>
        <NumberFormatter
          decimalScale={1}
          suffix={[scale.suffix, suffix].filter(Boolean).join('')}
          value={value / scale.min}
        />
      </Title>
      <Text textWrap="nowrap">{label}</Text>
    </Stack>
  )
}
