import { AreaChart } from '@mantine/charts'
import {
  ActionIcon,
  Anchor,
  Avatar,
  Box,
  Button,
  Card,
  Center,
  Divider,
  Group,
  Image,
  Indicator,
  Menu,
  NumberFormatter,
  Paper,
  Progress,
  Select,
  Stack,
  Text,
  Title,
} from '@mantine/core'
import { ArrowSquareOutIcon, CaretDownIcon } from '@phosphor-icons/react'
import { Link, useNavigate } from 'react-router'

import { safeNavigate } from '../util'

export const CardPage = () => {
  const navigate = useNavigate()

  return (
    <Stack gap="xl" h="100dvh" justify="stretch" p="xl" w="100%">
      <Anchor w="fit-content" onClick={() => safeNavigate(navigate, -1, '/browse')}>
        {'<-'} Back
      </Anchor>
      <Group align="start">
        <CardSection />
        <InfoSection />
      </Group>
      <RatingsSection />
    </Stack>
  )
}

const CardSection = () => (
  <Stack align="center" flex={1} miw="fit-content">
    <Paper
      radius={'15px'}
      shadow="lg"
      style={{
        overflow: 'clip',
      }}
    >
      <Image
        src="https://cards.scryfall.io/large/front/0/3/036ef8c9-72ac-46ce-af07-83b79d736538.jpg?1562730661"
        w={320}
      />
    </Paper>
    <Button
      component={Link}
      rightSection={<ArrowSquareOutIcon />}
      to="https://scryfall.com/card/9ed/100/storm-crow"
      w={'fit-content'}
    >
      View on Scryfall
    </Button>
  </Stack>
)

const InfoSection = () => (
  <Stack flex={3} h={'100%'}>
    <Title order={1}>Storm Crow</Title>
    <Text c="dimmed" maw={540} size="xl">
      Community Power Score
    </Text>
    <Group wrap="nowrap">
      <Title order={1} textWrap="nowrap">
        <NumberFormatter suffix={' pts'} value={10} />
      </Title>
      <Divider orientation="vertical" />
      <Title order={1} textWrap="nowrap">
        <NumberFormatter suffix={'%'} value={0.0002} />
      </Title>
    </Group>
    <Stack align="end" gap={'xs'} w={'100%'}>
      <Text>Insta-Ban</Text>
      <Progress value={70} w={'100%'} />
    </Stack>
    <Title order={2}>Rank #46 Overall</Title>
    <Text c="dimmed">
      <NumberFormatter suffix={' ratings'} value={23} />
    </Text>
    <AreaChart
      curveType="monotone"
      data={[
        {
          '%': 0.0,
          ratings: 200,
        },
        {
          '%': 0.1,
          ratings: 50,
        },
        {
          '%': 0.2,
          ratings: 30,
        },
        {
          '%': 0.3,
          ratings: 20,
        },
        {
          '%': 0.4,
          ratings: 400,
        },
        {
          '%': 0.5,
          ratings: 60,
        },
        {
          '%': 0.6,
          ratings: 32,
        },
        {
          '%': 0.7,
          ratings: 20,
        },
      ]}
      dataKey="%"
      h={240}
      series={[{ name: 'ratings', color: 'var(--mantine-primary-color-filled)' }]}
    />
  </Stack>
)

const RatingsSection = () => (
  <Stack>
    <Group w={'100%'}>
      <Title order={1}>Ratings</Title>
      <Select
        data={['👍 Most Liked', '👎 Most Disliked', '🔥 Most Controversial', '⏲️ Most Recent']}
        placeholder="sort by"
      />
    </Group>
    <Stack>
      <Indicator
        color="rgba(0,0,0,0)"
        label={
          <Box pos={'relative'} w={0}>
            <Button.Group pos="absolute" right={0} style={{ transform: 'translate(10%, -50%)' }}>
              <Button size="compact-md" variant="light">
                10 👍
              </Button>
              <Button size="compact-md" variant="default">
                5 👎
              </Button>
            </Button.Group>
          </Box>
        }
        position="bottom-end"
        size={32}
      >
        <Card withBorder orientation="horizontal" padding="sm">
          <Card.Section withBorder p="md">
            <Center h="100%">
              <Stack align="start" gap={'lg'}>
                <Group wrap="nowrap">
                  <Title order={2} textWrap="nowrap">
                    <NumberFormatter suffix={' pts'} value={10} />
                  </Title>
                  <Divider orientation="vertical" />
                  <Title order={2} textWrap="nowrap">
                    <NumberFormatter suffix={'%'} value={0.0002} />
                  </Title>
                </Group>
                <Group gap="sm">
                  <Avatar name="Benjamin Tarr" size="md" />
                  <Group gap={0}>
                    <Text size="md">Benjamin Tarr</Text>
                    <Menu position="bottom-end">
                      <Menu.Target>
                        <ActionIcon variant="transparent">
                          <CaretDownIcon />
                        </ActionIcon>
                      </Menu.Target>
                      <Menu.Dropdown>
                        <Menu.Item>View Profile</Menu.Item>
                      </Menu.Dropdown>
                    </Menu>
                  </Group>
                </Group>
              </Stack>
            </Center>
          </Card.Section>

          <Card.Section p="md">
            <Text>This card can be pitched to force of will so it gets the dub.</Text>
          </Card.Section>
        </Card>
      </Indicator>
    </Stack>
  </Stack>
)
