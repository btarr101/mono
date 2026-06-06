import {
  Anchor,
  BackgroundImage,
  Center,
  Divider,
  Group,
  NumberFormatter,
  Paper,
  Stack,
  Text,
  Title,
} from '@mantine/core'
import { Link } from 'react-router'

import type { Card } from '../types/bindings/Card'

export type MtgCardButtonProps = {
  card: Card
}

export const MtgCardButton = ({ card }: MtgCardButtonProps) => (
  <BackgroundImage
    radius="lg"
    src={
      card.image_uri ||
      'https://cards.scryfall.io/large/front/0/3/036ef8c9-72ac-46ce-af07-83b79d736538.jpg?1562730661'
    }
    style={{
      overflow: 'clip',
      aspectRatio: '672 / 936',
      width: 'fit-content',
    }}
  >
    <Stack align="center" h="100%" justify="end" p="md">
      <Center>
        <Paper withBorder p="md" radius="lg" shadow="xl" w="fit-content">
          <Stack gap={'xs'}>
            <Group wrap="nowrap">
              <Title order={4} textWrap="nowrap">
                <NumberFormatter suffix={' pts'} value={10} />
              </Title>
              <Divider orientation="vertical" />
              <Title order={4} textWrap="nowrap">
                <NumberFormatter suffix={'%'} value={0.0002} />
              </Title>
            </Group>
            <Group justify="space-between">
              <Text c="dimmed" size="xs">
                <NumberFormatter suffix={' ratings'} value={23} />
              </Text>
              <Anchor component={Link} flex={1} ta="center" to={`/browse/${card.oracle_id}`}>
                View
              </Anchor>
            </Group>
          </Stack>
        </Paper>
      </Center>
    </Stack>
  </BackgroundImage>
)
