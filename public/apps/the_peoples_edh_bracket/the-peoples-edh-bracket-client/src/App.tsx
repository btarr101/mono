import { useState } from 'react'
import { createBrowserRouter } from 'react-router'

const router = createBrowserRouter([
  {
    path: '/',
    element: <></>,
  },
])

export const App = () => {
  const [count, setCount] = useState(0)

  return <></>
}
