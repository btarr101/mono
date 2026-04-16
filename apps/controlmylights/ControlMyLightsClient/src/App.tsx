// app/layout.tsx (Next.js) or pages/_app.tsx or index.tsx
import 'react-beautiful-color/dist/react-beautiful-color.css'

import { ColorPicker } from 'react-beautiful-color'

import { EaselProvider, useEasel } from './contexts/EaselContext'

export const App = () => {
  return (
    <div className="flex flex-col w-full h-full bg-red-300 space-y-8">
      <h1 className="text-5xl text-center">Control My Lights</h1>
      <div className="mx-auto">
        <EaselProvider
          initialSplotchColors={[
            {
              red: 255,
              green: 255,
              blue: 255,
            },
            {
              red: 255,
              green: 255,
              blue: 255,
            },
            {
              red: 255,
              green: 255,
              blue: 255,
            },
            {
              red: 255,
              green: 255,
              blue: 255,
            },
          ]}
        >
          <Easel />
        </EaselProvider>
      </div>
    </div>
  )
}

const Easel = () => {
  const { activeSplotch, splotches } = useEasel()
  if (!activeSplotch) return

  return (
    <div>
      <ColorPicker
        color={{
          type: 'rgb',
          r: activeSplotch.color.red,
          g: activeSplotch.color.green,
          b: activeSplotch.color.blue,
        }}
        onChange={color => {
          const { r, g, b } = color.getRgb()
          activeSplotch.setColor({
            red: r,
            green: g,
            blue: b,
          })
        }}
      >
        <ColorPicker.Saturation className="flex-1 mb-3" />
        <div className="flex items-center gap-3 p-3 pt-0">
          <div className="flex-1 flex flex-col gap-3">
            <ColorPicker.Hue className="h-4" />
          </div>
        </div>
      </ColorPicker>
      <div>
        {splotches.map((splotch, index) => (
          <div
            key={index}
            style={{
              background: `rgb(${splotch.color.red},${splotch.color.green},${splotch.color.blue})`,
            }}
            onClick={splotch.setActive}
          >
            SPLOTCH-({index})
          </div>
        ))}
      </div>
    </div>
  )
}
