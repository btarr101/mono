import '@mantine/core/styles.css'
import '@mantine/charts/styles.css'
import '@mantine/notifications/styles.css'

import { MantineProvider } from '@mantine/core'
import { Notifications, notifications } from '@mantine/notifications'
import { GoogleOAuthProvider } from '@react-oauth/google'
import { MutationCache, QueryCache, QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'
import { HTTPError } from 'ky'
import { createBrowserRouter, RouterProvider } from 'react-router'

import { getCard } from './api/cards'
import { getHomeMetrics } from './api/home'
import { useAuthState } from './hooks/useAuth'
import { Layout } from './Layout'
import { BrowsePage } from './pages/BrowsePage'
import { CardPage } from './pages/CardPage'
import { HomePage } from './pages/HomePage'
import { theme } from './theme'

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
        loader: getHomeMetrics,
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

export const App = () => {
  const [, setAuthState] = useAuthState()
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
      onError: error => {
        if (error instanceof HTTPError && error.response.status === 401) {
          setAuthState({ ty: null })
          queryClient.clear()
        }

        notifications.show({
          title: error.name,
          message: error.message,
          color: 'red',
          autoClose: false,
        })
      },
    }),
    mutationCache: new MutationCache({
      onError: error => {
        if (error instanceof HTTPError && error.response.status === 401) {
          setAuthState({ ty: null })
          queryClient.clear()
        }

        notifications.show({
          title: error.name,
          message: error.message,
          color: 'red',
          autoClose: false,
        })
      },
    }),
  })

  return (
    <GoogleOAuthProvider clientId={import.meta.env.VITE_GOOGLE_CLIENT_ID}>
      <QueryClientProvider client={queryClient}>
        <MantineProvider theme={theme}>
          <RouterProvider router={router} />
          <Notifications />
        </MantineProvider>
        <ReactQueryDevtools initialIsOpen={false} />
      </QueryClientProvider>
    </GoogleOAuthProvider>
  )
}
