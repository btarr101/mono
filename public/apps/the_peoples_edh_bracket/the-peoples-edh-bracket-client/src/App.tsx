import '@mantine/core/styles.css'
import '@mantine/charts/styles.css'
import '@mantine/notifications/styles.css'

import { MantineProvider } from '@mantine/core'
import { Notifications, notifications } from '@mantine/notifications'
import { GoogleOAuthProvider } from '@react-oauth/google'
import { MutationCache, QueryCache, QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'
import { HTTPError } from 'ky'
import { createBrowserRouter, redirect, RouterProvider } from 'react-router'

import { getCard } from './api/cards'
import { getTrackedDeck } from './api/decks'
import { getHomeMetrics } from './api/home'
import { getPerson } from './api/persons'
import { useAuthState } from './hooks/useAuth'
import { Layout } from './Layout'
import { AboutPage } from './pages/AboutPage'
import { AnalyzePage } from './pages/AnalyzePage'
import { BrowsePage } from './pages/BrowsePage'
import { CardPage } from './pages/CardPage'
import { CommunityPage } from './pages/CommunityPage'
import { HomePage } from './pages/HomePage'
import { NewAnalyzedDeckPage } from './pages/NewAnalyzedDeckPage'
import { ProfilePage } from './pages/ProfilePage'
import { TrackedDeckPage } from './pages/TrackedDeckPage'
import { theme } from './theme'
import { readNewAnalyzedDeck } from './util/analyzed-deck'

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
        children: [
          {
            index: true,
            Component: AnalyzePage,
          },
          {
            path: 'new',
            Component: NewAnalyzedDeckPage,
            loader: () => {
              const analyzedDeck = readNewAnalyzedDeck()
              if (!analyzedDeck) throw redirect('/analyze')
              return { analyzedDeck }
            },
          },
          {
            path: ':uuid',
            Component: TrackedDeckPage,
            loader: async ({ params }) => {
              if (!params.uuid) throw new Error('no uuid provided')
              const trackedDeck = await getTrackedDeck(params.uuid)
              return { trackedDeck }
            },
          },
        ],
      },
      {
        path: '/community',
        children: [
          {
            index: true,
            Component: CommunityPage,
          },
          {
            path: ':uuid',
            Component: ProfilePage,
            loader: async ({ params }) => {
              if (!params.uuid) throw new Error('no uuid provided')
              const person = await getPerson(params.uuid)
              return { person }
            },
          },
        ],
      },
      {
        path: '/about',
        Component: AboutPage,
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
