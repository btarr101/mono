import { Anchor, Divider, List, Stack, Text, Title } from '@mantine/core'

export const AboutPage = () => (
  <Stack gap="lg" mih="100dvh" p="xl" w="100%">
    <Title size="4rem">
      About the{' '}
      <Text inherit c="var(--mantine-primary-color-filled)" component="span">
        people{"'"}s
      </Text>{' '}
      edh bracket.
    </Title>

    <Text c="dimmed" size="xl">
      <em>
        “We all agreed to play 4s. My 4 was a sweet Yorion blink deck. Their 4 was Thassa’s Oracle +
        Demonic Consultation.”
      </em>
    </Text>

    <Text>
      If you play Commander long enough, you’ve lived this mismatch. Even with better language
      around deck strength, “power level” is still subjective. Two players can say the same number
      and mean very different things.
    </Text>

    <Text>
      The official Commander Brackets beta is a strong step toward common vocabulary, but it’s still
      a curated system.{' '}
      <strong>The People’s EDH Bracket exists to add a data layer to that conversation.</strong>
    </Text>

    <Text>
      Instead of asking only “what do experts think this card does to a game?”, this project asks
      what the community thinks across real tables, metas, and experience levels.
    </Text>

    <Divider />

    <Title order={2}>How it works</Title>

    <Text>
      Each player assigns <strong>personal points (ppts)</strong> to cards on a fixed{' '}
      <strong>0–10 scale</strong>. A card’s global score (<strong>pts</strong>) is the average of
      everyone’s ppts for that card.
    </Text>

    <List spacing="xs">
      <List.Item>0 means “not powerful in Commander”; 10 means “format-defining power.”</List.Item>
      <List.Item>Every unrated card is implicitly 0 ppts in deck analysis.</List.Item>
      <List.Item>Your ppts are capped at 10 per card.</List.Item>
    </List>

    <Text>
      This draws inspiration from the Canadian Highlander points philosophy:{' '}
      <Anchor href="https://canadianhighlander.ca/points-list/" rel="noreferrer" target="_blank">
        canadianhighlander.ca/points-list
      </Anchor>
      .
    </Text>

    <Title order={3}>Quick example</Title>

    <Text>
      John assigns 5 ppts to Storm Crow and 9 ppts to Force of Will. Will assigns 1 ppts to Storm
      Crow.
    </Text>

    <List spacing="xs">
      <List.Item>Storm Crow ppts: 5 and 1 → global score = 3.0 pts.</List.Item>
      <List.Item>Force of Will ppts: 9 and unrated (0) → global score = 4.5 pts.</List.Item>
      <List.Item>All ratings are now on the same fixed 0–10 scale.</List.Item>
    </List>

    <Divider />

    <Title order={2}>Basic usage</Title>

    <List spacing="sm">
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
        <strong>Rate cards:</strong> submit your own 0–10 rating (and optional reasoning) to feed
        the model.
      </List.Item>
      <List.Item>
        <strong>Analyze:</strong> paste a decklist to get a community-driven estimate of deck power.
      </List.Item>
      <List.Item>
        <strong>Community:</strong> explore users and compare how people evaluate cards.
      </List.Item>
    </List>

    <Text>
      This site doesn’t replace Rule 0. It helps make Rule 0 conversations faster and clearer by
      adding shared, crowd-sourced data.
    </Text>
  </Stack>
)
