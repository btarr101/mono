import '@mantine/core/styles.css'
import '@mantine/charts/styles.css'

import { MantineProvider } from '@mantine/core'
import { createBrowserRouter, RouterProvider } from 'react-router'

import { Layout } from './Layout'
import { BrowsePage } from './pages/BrowsePage'
import { CardPage } from './pages/CardPage'
import { HomePage } from './pages/HomePage'
import { theme } from './theme'

const router = createBrowserRouter([
  {
    path: '/',
    Component: Layout,
    children: [
      {
        index: true,
        Component: HomePage,
      },
      {
        path: '/browse',
        children: [
          {
            index: true,
            Component: BrowsePage,
          },
          {
            path: ':oracleId',
            Component: CardPage,
          },
        ],
      },
      {
        path: '/analyze',
        Component: HomePage,
      },
      {
        path: '/community',
        Component: HomePage,
      },
      {
        path: '/about',
        Component: HomePage,
      },
    ],
  },
])

export const App = () => (
  <MantineProvider theme={theme}>
    <RouterProvider router={router} />
  </MantineProvider>
)
