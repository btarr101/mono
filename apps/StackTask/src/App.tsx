import { type CSSProperties } from 'react'
import { StackProvider } from './contexts/StackContext/StackProvider'
import { Stack } from './Stack'

export const App = () => (
  <StackProvider>
    <div style={screenStyle}>
      <Stack />
    </div>
  </StackProvider>
)

const screenStyle: CSSProperties = {
  minHeight: '100vh',
  display: 'flex',
  flexDirection: 'column',
  alignItems: 'stretch',
}
