import { Box } from '@mantine/core'

import { LoadingImage } from './LoadingImage'

export type TableCardProps = {
  imageUri?: string | null
}

export const TableCard = ({ imageUri }: TableCardProps) => (
  <Box h={45} pos="relative" w={32}>
    <LoadingImage bdrs={1} h={45} left={0} pos="absolute" src={imageUri} top={0} w={32} />
  </Box>
)
