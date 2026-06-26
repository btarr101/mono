import {
  Box,
  Button,
  Card,
  Center,
  Divider,
  Group,
  Indicator,
  NumberInput,
  Stack,
  Text,
  Textarea,
  Title,
} from '@mantine/core'
import { hasLength, useForm } from '@mantine/form'
import { ShareIcon } from '@phosphor-icons/react'
import { useMemo } from 'react'

import { PointsNumberFormatter } from '../../components/PointsNumberFormatter'
import { useMe } from '../../hooks/usePersons'
import type { CardRatingEnriched } from '../../types/bindings/CardRatingEnriched'
import { formatTimeStamp } from '../../util'

type RatingInputProps = {
  rating: CardRatingEnriched | null
  onShare?: () => void
  onSave: (values: { points: number | null; reason: string | null }) => Promise<void>
}

export const RatingInput = ({ rating, onSave, onShare }: RatingInputProps) => {
  const { data: me } = useMe()
  const form = useForm({
    mode: 'controlled',
    initialValues: {
      points: rating ? Number(rating.points) : null, // todo: look into bigfloat impls
      reason: rating?.reason ?? '',
    },
    validate: {
      reason: hasLength({ max: 300 }, 'Reason must be less 300 characters or less'),
    },
  })

  const totalPoints = useMemo(() => {
    const total_points = rating?.total_points ?? me?.total_points
    if (total_points === undefined) return undefined

    const points = form.getValues().points ?? 0

    return parseFloat(total_points) - parseFloat(rating?.points ?? '0') + points
  }, [form, rating, me])

  return (
    <form
      onSubmit={form.onSubmit(async values => {
        await onSave(values)
        form.resetDirty(values)
      })}
    >
      <Indicator
        color="transparent"
        label={
          <Box pos={'relative'} w={0}>
            <Group
              pos="absolute"
              right={0}
              style={{ transform: 'translate(10%, -50%)' }}
              wrap="nowrap"
            >
              {rating !== null && (
                <Button.Group>
                  <Button
                    disabled
                    size="compact-md"
                    style={{ pointerEvents: 'none' }}
                    variant="default"
                  >
                    {rating.reviews.likes} 👍
                  </Button>
                  <Button
                    disabled
                    size="compact-md"
                    style={{ pointerEvents: 'none' }}
                    variant="default"
                  >
                    {rating.reviews.dislikes} 👎
                  </Button>
                  <Button disabled={!onShare} size="compact-md" variant="default" onClick={onShare}>
                    <ShareIcon />
                  </Button>
                </Button.Group>
              )}
              <Button
                disabled={!form.isDirty()}
                loading={form.submitting}
                miw={'fit-content'}
                type="submit"
              >
                Save
              </Button>
            </Group>
          </Box>
        }
        position="bottom-end"
        size={32}
      >
        <Card withBorder orientation="horizontal" padding="sm">
          <Card.Section withBorder mih={'125px'} p="md" style={{ alignSelf: 'stretch' }}>
            <Center h="100%">
              <Group wrap="nowrap">
                <Title order={2} textWrap="nowrap">
                  <PointsNumberFormatter points={rating?.global_points ?? '0.0'} suffix=" pts" />
                </Title>
                <Divider orientation="vertical" />
                <Stack gap="xs">
                  <NumberInput
                    key={form.key('points')}
                    min={0}
                    placeholder="0 ppts"
                    size="lg"
                    styles={{
                      input: {
                        fieldSizing: 'content',
                        paddingRight:
                          'calc(var(--input-right-section-width) + var(--mantine-spacing-md))',
                      },
                    }}
                    suffix=" ppts"
                    {...form.getInputProps('points')}
                  />
                  <Divider />
                  <Text span c="dimmed" size="sm">
                    <PointsNumberFormatter points={totalPoints ?? '0'} suffix=" ppts" />
                  </Text>
                </Stack>
              </Group>
            </Center>
          </Card.Section>
          <Card.Section flex={1}>
            <Textarea
              autosize
              key={form.key('reason')}
              placeholder="Enter a reason..."
              styles={{
                input: {
                  border: 'none',
                  borderRadius: 0,
                  fontSize: 'var(--mantine-font-size-md)',
                  padding: 'var(--mantine-spacing-md)',
                  resize: 'none',
                },
              }}
              w="100%"
              {...form.getInputProps('reason')}
            />
          </Card.Section>
        </Card>
      </Indicator>
      {rating && (
        <Text c="dimmed" px="xs" size="xs">
          {formatTimeStamp(rating.created_at)}
          {rating.updated_at && ' • edited'}
        </Text>
      )}
    </form>
  )
}
