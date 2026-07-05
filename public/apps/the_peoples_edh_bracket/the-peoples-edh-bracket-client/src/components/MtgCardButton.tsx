import {
  Anchor,
  Center,
  Divider,
  Group,
  NumberFormatter,
  Paper,
  Skeleton,
  Stack,
  Text,
  Title,
} from '@mantine/core'
import { Link } from 'react-router'

import type { CardWithMetrics } from '../types/bindings/CardWithMetrics'
import { LoadingImage } from './LoadingImage'
import { CARD_BUTTON_DIMENSIONS } from './MtgCardButton.constants'

export { CARD_BUTTON_DIMENSIONS } from './MtgCardButton.constants'

export type MtgCardButtonProps = {
  card: CardWithMetrics
}

export const MtgCardButton = ({ card }: MtgCardButtonProps) => {
  return (
    <Stack
      align="center"
      justify="end"
      p="md"
      {...CARD_BUTTON_DIMENSIONS}
      bdrs="md"
      pos="relative"
      style={{
        overflow: 'clip',
      }}
    >
      <LoadingImage
        pos="absolute"
        src={
          card.image_uri ||
          'https://cards.scryfall.io/large/front/0/3/036ef8c9-72ac-46ce-af07-83b79d736538.jpg?1562730661'
        }
        top={0}
        {...CARD_BUTTON_DIMENSIONS}
      />
      <Center>
        <Paper withBorder opacity={0.9} p="md" radius="lg" shadow="xl" w="fit-content">
          <Stack gap={'xs'}>
            <Group wrap="nowrap">
              <Title order={4} textWrap="nowrap">
                <NumberFormatter
                  decimalScale={2}
                  fixedDecimalScale={true}
                  suffix={' pts'}
                  value={card.global_points}
                />
              </Title>
              <Divider orientation="vertical" />
              <Title order={5} textWrap="nowrap">
                <NumberFormatter prefix="Rank #" value={card.card_rank} />
              </Title>
            </Group>
            <Group justify="space-between">
              <Text c="dimmed" size="xs">
                <NumberFormatter suffix={' ratings'} value={card.total_ratings} />
              </Text>
              <Anchor component={Link} flex={1} ta="center" to={`/browse/${card.oracle_id}`}>
                View
              </Anchor>
            </Group>
          </Stack>
        </Paper>
      </Center>
    </Stack>
  )
}

export const MtgCardButtonGhost = () => (
  <Skeleton
    style={{ aspectRatio: '672 / 936', width: 'fit-content' }}
    {...CARD_BUTTON_DIMENSIONS}
  />
)
