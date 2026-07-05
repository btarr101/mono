import { Paper, Text, Title } from '@mantine/core'

export type EmptyPlaceholderProps = {
  title: string
  subText: string
}

export const EmptyPlaceholder = ({ title, subText }: EmptyPlaceholderProps) => (
  <Paper withBorder h="100%" p="lg" w="100%">
    <Title order={3}>{title}</Title>
    <Text c="dimmed" maw={540} size="xl">
      {subText}
    </Text>
  </Paper>
)
