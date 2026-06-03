import '@mantine/core/styles.css'

import { MantineProvider } from '@mantine/core'
import { createBrowserRouter, RouterProvider } from 'react-router'

import { Layout } from './Layout'
import { Home } from './pages/Home'
import { theme } from './theme'

const router = createBrowserRouter([
  {
    path: '/',
    Component: Layout,
    children: [
      {
        index: true,
        Component: Home,
      },
      {
        path: '/browse',
        Component: Home,
      },
      {
        path: '/analyze',
        Component: Home,
      },
      {
        path: '/community',
        Component: Home,
      },
      {
        path: '/about',
        Component: Home,
      },
    ],
  },
])

export const App = () => (
  <MantineProvider theme={theme}>
    <RouterProvider router={router} />
  </MantineProvider>
)
