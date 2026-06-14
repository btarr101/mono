import {
  Box,
  Button,
  Card,
  Center,
  Divider,
  Group,
  Indicator,
  NumberFormatter,
  NumberInput,
  Stack,
  Text,
  Textarea,
  Title,
} from '@mantine/core'
import { hasLength, useForm } from '@mantine/form'
import { ShareIcon } from '@phosphor-icons/react'

import type { CardRatingWithReviewsAndGlobalPoints } from '../../types/bindings/CardRatingWithReviewsAndGlobalPoints'
import { formatTimeStamp } from '../../util'
import { useMemo } from 'react'

type RatingInputProps = {
  rating: CardRatingWithReviewsAndGlobalPoints | null
  onShare?: () => void
  onSave: (values: { points: number | null; reason: string | null }) => Promise<void>
}

export const RatingInput = ({ rating, onSave, onShare }: RatingInputProps) => {
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
    if (rating?.total_points === undefined) return undefined
    const points = form.getValues().points
    if (points === null) return undefined

    return parseFloat(rating.total_points) - parseFloat(rating.points) + points
  }, [form, rating])

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
                  <NumberFormatter
                    decimalScale={2}
                    fixedDecimalScale={true}
                    suffix={' pts'}
                    value={rating?.global_points ?? '0.0'}
                  />
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
                  <Text size="sm" span c="dimmed">
                    <NumberFormatter
                      decimalScale={2}
                      fixedDecimalScale={true}
                      suffix={' ppts'}
                      value={totalPoints}
                    />
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
