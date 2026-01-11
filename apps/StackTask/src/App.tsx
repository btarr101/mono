import { type CSSProperties } from 'react'
import { StackProvider } from './contexts/StackContext/StackProvider'
import { Stack } from './Stack'
import { ContextMenuProvider } from './contexts/ContextMenuContext/ContextMenuProvider'

export const App = () => (
  <ContextMenuProvider>
    <StackProvider>
      <div style={screenStyle}>
        <Stack />
      </div>
    </StackProvider>
  </ContextMenuProvider>
)

const screenStyle: CSSProperties = {
  position: 'fixed',
  inset: 0,
  width: '100vw',
  height: '100vh',
  background: 'red',
  display: 'flex',
  justifyContent: 'center',
  alignItems: 'center',
}
