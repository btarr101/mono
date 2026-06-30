import { Anchor, Divider, List, Stack, Text, Title } from '@mantine/core'

export const AboutPage = () => {
  return (
    <Stack gap="lg" mih="100dvh" p="xl" w="100%">
      <Title size="4rem">
        About the{' '}
        <Text inherit c="var(--mantine-primary-color-filled)" component="span">
          people{"'"}s
        </Text>{' '}
        edh bracket.
      </Title>

      <Text c="dimmed" maw={900} size="xl">
        <em>
          “We all agreed to play 4s. My 4 was a sweet Yorion blink deck. Their 4 was Thassa’s Oracle
          + Demonic Consultation.”
        </em>
      </Text>

      <Text maw={900}>
        If you play Commander long enough, you’ve lived this mismatch. Even with better language
        around deck strength, “power level” is still subjective. Two players can say the same number
        and mean very different things.
      </Text>

      <Text maw={900}>
        The official Commander Brackets beta is a strong step toward common vocabulary, but it’s
        still a curated system.{' '}
        <strong>The People’s EDH Bracket exists to add a data layer to that conversation.</strong>
      </Text>

      <Text maw={900}>
        Instead of asking only “what do experts think this card does to a game?”, this project asks
        what the community thinks across real tables, metas, and experience levels.
      </Text>

      <Divider />

      <Title order={2}>How it works</Title>

      <Text maw={900}>
        Each player assigns <strong>personal points (ppts)</strong> to cards based on their own
        scale. Those points are then normalized into a <strong>0–10 personal scale (pts)</strong>,
        and each card’s global score is the average of everyone’s normalized score.
      </Text>

      <List maw={900} spacing="xs">
        <List.Item>Power is treated like a limited budget you distribute across cards.</List.Item>
        <List.Item>You can’t meaningfully make everything a 10.</List.Item>
        <List.Item>Every unrated card is implicitly 0 ppts.</List.Item>
      </List>

      <Text maw={900}>
        This draws inspiration from the Canadian Highlander points philosophy:{' '}
        <Anchor href="https://canadianhighlander.ca/points-list/" rel="noreferrer" target="_blank">
          canadianhighlander.ca/points-list
        </Anchor>
        .
      </Text>

      <Title order={3}>Quick example</Title>

      <Text maw={900}>
        John assigns 5 ppts to Storm Crow and 5 ppts to Force of Will. Will assigns 1 ppts to Storm
        Crow.
      </Text>

      <List maw={900} spacing="xs">
        <List.Item>John total = 10 ppts → Storm Crow = 5.0 pts, Force of Will = 5.0 pts.</List.Item>
        <List.Item>Will total = 1 ppts → Storm Crow = 10.0 pts.</List.Item>
        <List.Item>
          Global averages: Storm Crow = (5.0 + 10.0) / 2 = 7.5 pts, Force of Will = (5.0 + 0.0) / 2
          = 2.5 pts.
        </List.Item>
      </List>

      <Divider />

      <Title order={2}>Basic usage</Title>

      <List maw={900} spacing="sm">
        <List.Item>
          <strong>Home:</strong> search cards quickly and see trending activity.
        </List.Item>
        <List.Item>
          <strong>Browse:</strong> sort by highest rated, lowest rated, most controversial, most
          rated, least rated, or trending.
        </List.Item>
        <List.Item>
          <strong>Card pages:</strong> view a card’s community score, rank, and rating distribution.
        </List.Item>
        <List.Item>
          <strong>Rate cards:</strong> submit your own points (and optional reasoning) to feed the
          model.
        </List.Item>
        <List.Item>
          <strong>Analyze:</strong> paste a decklist to get a community-driven estimate of deck
          power.
        </List.Item>
        <List.Item>
          <strong>Community:</strong> explore users and compare how people evaluate cards.
        </List.Item>
      </List>

      <Text maw={900}>
        This site doesn’t replace Rule 0. It helps make Rule 0 conversations faster and clearer by
        adding shared, crowd-sourced data.
      </Text>
    </Stack>
  )
}
