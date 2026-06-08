import '@mantine/core/styles.css'
import '@mantine/charts/styles.css'
import '@mantine/notifications/styles.css'

import { MantineProvider } from '@mantine/core'
import { Notifications, notifications } from '@mantine/notifications'
import { MutationCache, QueryCache, QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'
import { createBrowserRouter, RouterProvider } from 'react-router'

import { getCard } from './api/cards'
import { Layout } from './Layout'
import { BrowsePage } from './pages/BrowsePage'
import { CardPage } from './pages/CardPage'
import { HomePage } from './pages/HomePage'
import { theme } from './theme'

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      throwOnError: false,
    },
    mutations: {
      throwOnError: false,
    },
  },
  queryCache: new QueryCache({
    onError: error =>
      notifications.show({
        title: error.name,
        message: error.message,
        color: 'red',
        autoClose: false,
      }),
  }),
  mutationCache: new MutationCache({
    onError: error =>
      notifications.show({
        title: error.name,
        message: error.message,
        color: 'red',
        autoClose: false,
      }),
  }),
})

const router = createBrowserRouter([
  {
    path: '/',
    Component: Layout,
    // TODO: THIS IS NOT PERMANENT!
    hydrateFallbackElement: <p>Loading...</p>,
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
            loader: async ({ params }) => {
              if (!params.oracleId) throw new Error('no oracleId provided')
              const card = await getCard(params.oracleId)

              return { card }
            },
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
  <QueryClientProvider client={queryClient}>
    <MantineProvider theme={theme}>
      <RouterProvider router={router} />
      <Notifications />
    </MantineProvider>
    <ReactQueryDevtools initialIsOpen={false} />
  </QueryClientProvider>
)
