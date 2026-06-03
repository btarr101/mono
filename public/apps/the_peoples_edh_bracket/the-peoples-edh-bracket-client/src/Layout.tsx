import { AppShell, NavLink, Stack, Text, Title } from '@mantine/core'
import {
  ChartLineIcon,
  HandFistIcon,
  HouseIcon,
  InfoIcon,
  MagnifyingGlassIcon,
  UsersThreeIcon,
} from '@phosphor-icons/react'
import { Outlet } from 'react-router'
import { NavLink as RouterNavLink } from 'react-router'

export const Layout = () => (
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
        renderRoot={props => <RouterNavLink end to="/" {...props} />}
      />
      <NavLink
        label="Browse"
        leftSection={<MagnifyingGlassIcon />}
        renderRoot={props => <RouterNavLink end to="/browse" {...props} />}
      />
      <NavLink
        label="Analyze"
        leftSection={<ChartLineIcon />}
        renderRoot={props => <RouterNavLink end to="/analyze" {...props} />}
      />
      <NavLink
        label="Community"
        leftSection={<UsersThreeIcon />}
        renderRoot={props => <RouterNavLink end to="/community" {...props} />}
      />
      <NavLink
        label="About"
        leftSection={<InfoIcon />}
        renderRoot={props => <RouterNavLink end to="/about" {...props} />}
      />
    </AppShell.Navbar>
    <AppShell.Main>
      <Outlet />
    </AppShell.Main>
  </AppShell>
)
