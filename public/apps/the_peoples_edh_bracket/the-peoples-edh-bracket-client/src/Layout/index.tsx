import { AppShell, Box, Menu, NavLink, Space, Stack, Text, Title } from '@mantine/core'
import { useDisclosure } from '@mantine/hooks'
import {
  ChartLineIcon,
  HandFistIcon,
  HouseIcon,
  InfoIcon,
  MagnifyingGlassIcon,
  UsersThreeIcon,
} from '@phosphor-icons/react'
import { NuqsAdapter } from 'nuqs/adapters/react-router/v7'
import { useState } from 'react'
import { Link, ScrollRestoration } from 'react-router'
import { Outlet } from 'react-router'
import { NavLink as RouterNavLink } from 'react-router'

import { PersonProfileLine } from '../components/PersonProfileLine'
import { useAuthState, useLogin, useLogout } from '../hooks/useAuth'
import { useDebugPostPerson, useMe, useSearchPersons } from '../hooks/usePersons'
import { LoginModal } from './LoginModal'

export const Layout = () => (
  <NuqsAdapter>
    <AppShell navbar={{ breakpoint: 'xs', width: 280 }} padding={0}>
      <AppShell.Navbar style={{ overflowY: 'auto' }}>
        <Stack align="center" gap="xs" p="lg">
          <HandFistIcon size={96} />
          <Title ta="center">
            the people{"'"}s
            <br />
            <Text inherit c="var(--mantine-primary-color-filled)" component="span">
              (edh)
            </Text>{' '}
            bracket
          </Title>
        </Stack>
        <NavLink
          label="Home"
          leftSection={<HouseIcon />}
          renderRoot={props => <RouterNavLink to="/" {...props} />}
        />
        <NavLink
          label="Browse"
          leftSection={<MagnifyingGlassIcon />}
          renderRoot={props => <RouterNavLink to="/browse" {...props} />}
        />
        <NavLink
          label="Analyze"
          leftSection={<ChartLineIcon />}
          renderRoot={props => <RouterNavLink to="/analyze" {...props} />}
        />
        <NavLink
          label="Community"
          leftSection={<UsersThreeIcon />}
          renderRoot={props => <RouterNavLink to="/community" {...props} />}
        />
        <NavLink
          label="About"
          leftSection={<InfoIcon />}
          renderRoot={props => <RouterNavLink to="/about" {...props} />}
        />
        <Space flex={1} />
        <DebugAuthSection />
      </AppShell.Navbar>
      <AppShell.Main>
        <Outlet />
      </AppShell.Main>
      <ScrollRestoration />
    </AppShell>
  </NuqsAdapter>
)

const DebugAuthSection = () => {
  const [authState] = useAuthState()

  const logout = useLogout()
  const debugPostPerson = useDebugPostPerson()
  const me = useMe()

  const [opened, { open, close }] = useDisclosure(false)

  return (
    <>
      <Box p={'md'}>
        <PersonProfileLine loading={me.isLoading} person={me.data}>
          <Menu.Item onClick={() => debugPostPerson.mutate()}>Create new user</Menu.Item>
          <Menu.Sub>
            <Menu.Sub.Target>
              <Menu.Sub.Item>Log into debug user</Menu.Sub.Item>
            </Menu.Sub.Target>
            <DebugUserDropdown />
          </Menu.Sub>
          {authState.ty === null && <Menu.Item onClick={open}>Log in</Menu.Item>}
          {authState.ty !== null && (
            <>
              <Menu.Item
                component={Link}
                disabled={!me.data?.uuid}
                to={`/community/${me.data?.uuid}`}
              >
                View Profile
              </Menu.Item>
              <Menu.Item color="red" onClick={logout}>
                Log out
              </Menu.Item>
            </>
          )}
        </PersonProfileLine>
      </Box>
      <LoginModal opened={opened} onClose={close} />
    </>
  )
}

const DebugUserDropdown = () => {
  const login = useLogin()

  const [q, setQ] = useState('')
  const persons = useSearchPersons(q || null)

  const allPersons = persons.data?.pages.flat()
  const requiredSpacers = 10 - (allPersons?.length ?? 0)

  return (
    <Menu.Sub.Dropdown>
      <Menu.Search
        placeholder="Search for user"
        value={q}
        onChange={event => setQ(event.currentTarget.value)}
      />
      {persons.data?.pages.flat().map(({ uuid, username }) => (
        <Menu.Item key={uuid} onClick={() => login({ ty: 'debug', personUUID: uuid })}>
          {username}
        </Menu.Item>
      ))}
      {Array.from({ length: requiredSpacers }).map((_, index) => (
        <Menu.Item disabled key={index}>
          &nbsp;
        </Menu.Item>
      ))}
    </Menu.Sub.Dropdown>
  )
}
