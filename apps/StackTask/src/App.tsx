import { type CSSProperties } from 'react'
import { StackProvider } from './contexts/StackContext/StackProvider'
import { Stack } from './Stack'
import { ContextMenuProvider } from './contexts/ContextMenuContext/ContextMenuProvider'

export const App = () => (
  <ContextMenuProvider>
    <StackProvider>
      <div style={screenStyle}>
        <h1
          style={{
            textAlign: 'center',
            alignContent: 'center',
            fontSize: '5rem',
            marginRight: 'auto',
            marginLeft: 'auto',
          }}
        >
          StackTask
        </h1>
        <Stack />
      </div>
    </StackProvider>
  </ContextMenuProvider>
)

const screenStyle: CSSProperties = {
  minHeight: '100vh',
  display: 'flex',
  flexDirection: 'column',
  alignItems: 'stretch',
}
