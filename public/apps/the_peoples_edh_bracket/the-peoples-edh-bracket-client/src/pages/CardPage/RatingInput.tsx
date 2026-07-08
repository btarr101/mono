import {
  Box,
  Button,
  Card,
  Center,
  Group,
  Indicator,
  NumberInput,
  Text,
  Textarea,
  useMatches,
} from '@mantine/core'
import { hasLength, useForm } from '@mantine/form'
import { ShareIcon } from '@phosphor-icons/react'

import type { CardRatingEnriched } from '../../types/bindings/CardRatingEnriched'
import { formatTimeStamp } from '../../util'

type RatingInputProps = {
  rating: CardRatingEnriched | null
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
      points: value => {
        if (value === null) return 'Rating is required'
        if (value < 0 || value > 10) return 'Rating must be between 0 and 10'
        return null
      },
      reason: hasLength({ max: 300 }, 'Reason must be less 300 characters or less'),
    },
  })

  const isMobile = useMatches({
    base: true,
    md: false,
  })

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
        <Card withBorder orientation={isMobile ? 'vertical' : 'horizontal'} padding="sm">
          <Card.Section withBorder mih={'125px'} p="md" style={{ alignSelf: 'stretch' }}>
            <Center h="100%">
              <NumberInput
                clampBehavior="strict"
                key={form.key('points')}
                max={10}
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
