import { AreaChart } from '@mantine/charts'
import {
  ActionIcon,
  Alert,
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
  NumberInput,
  Paper,
  Progress,
  Select,
  Stack,
  Text,
  Textarea,
  Title,
} from '@mantine/core'
import {
  ArrowSquareOutIcon,
  CaretDownIcon,
  InfoIcon,
  PushPinIcon,
  ShareIcon,
} from '@phosphor-icons/react'
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
      <Stack gap="sm">
        <Alert
          color="orange"
          icon={<InfoIcon />}
          title="You haven't rated this card yet."
          variant="light"
        />
        <RatingInput />
      </Stack>
      <RatingsSection />
    </Stack>
  )
}

const CardSection = () => (
  <Stack align="center" flex={1} h="100%" miw="fit-content">
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
    <Center flex={1}>
      <Button
        component={Link}
        rightSection={<ArrowSquareOutIcon />}
        target="_blank"
        to="https://scryfall.com/card/9ed/100/storm-crow"
        w={'fit-content'}
      >
        View on Scryfall
      </Button>
    </Center>
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
  <Stack pb="lg">
    <Group w={'100%'}>
      <Title order={1}>Community Ratings</Title>
      <Select
        allowDeselect={false}
        data={['👍 Most Liked', '👎 Most Disliked', '🔥 Most Controversial', '⏲️ Most Recent']}
        defaultValue="👍 Most Liked"
        placeholder="sort by"
      />
    </Group>
    <Stack gap="xl">
      <Rating pinned={true} />
      <Rating />
    </Stack>
  </Stack>
)

const RatingInput = () => (
  <Indicator
    color="transparent"
    label={
      <Box pos={'relative'} w={0}>
        <Group pos="absolute" right={0} style={{ transform: 'translate(10%, -50%)' }} wrap="nowrap">
          <Button.Group>
            <Button size="compact-md" style={{ pointerEvents: 'none' }} variant="default">
              10 👍
            </Button>
            <Button size="compact-md" style={{ pointerEvents: 'none' }} variant="default">
              5 👎
            </Button>
            <Button size="compact-md" variant="default">
              <ShareIcon />
            </Button>
          </Button.Group>
          <Button miw={'fit-content'}>Save</Button>
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
              <NumberFormatter suffix={' pts'} value={10} />
            </Title>
            <Divider orientation="vertical" />
            <NumberInput
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
            />
          </Group>
        </Center>
      </Card.Section>
      <Card.Section flex={1}>
        <Textarea
          autosize
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
        />
      </Card.Section>
    </Card>
  </Indicator>
)

type RatingProps = {
  pinned?: boolean
}

const Rating = ({ pinned }: RatingProps) => (
  <Box pos="relative">
    <Indicator
      color="transparent"
      label={
        <Box pos={'relative'} w={0}>
          <Button.Group pos="absolute" right={0} style={{ transform: 'translate(10%, -50%)' }}>
            <Button size="compact-md" variant="light">
              10 👍
            </Button>
            <Button size="compact-md" variant="default">
              5 👎
            </Button>
            <Button size="compact-md" variant="default">
              <ShareIcon />
            </Button>
            <Button size="compact-md" variant={pinned ? 'filled' : 'default'}>
              <PushPinIcon />
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
                <Title c="dimmed" order={4} textWrap="nowrap">
                  <NumberFormatter suffix={' ppts'} value={3.5} />
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
  </Box>
)
