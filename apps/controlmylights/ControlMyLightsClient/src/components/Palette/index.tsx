import './Palette.css'
import 'react-beautiful-color/dist/react-beautiful-color.css'

import type { CSSProperties } from 'react'
import { ColorPicker } from 'react-beautiful-color'
import { FaSplotch } from 'react-icons/fa6'

import { usePalette } from '../../contexts/PaletteContext'

const splotchRotations = Array.from({ length: 9 }).map(() => Math.random() * 360)

export const Palette = () => {
  const { activeSplotch, splotches } = usePalette()
  if (!activeSplotch) return

  return (
    <div className="flex flex-col h-full gap-4">
      <ColorPicker
        className="palette-picker h-full w-full aspect-square border-3 shadow-none"
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
        <ColorPicker.Saturation />
        <div className="items-center p-2">
          <ColorPicker.Hue className="h-4" />
        </div>
      </ColorPicker>
      <div className="grid h-full w-full max-w-full grid-cols-3 grid-rows-3 gap-2">
        {splotches.map((splotch, index) => {
          const style = {
            color: `rgb(${splotch.color.red},${splotch.color.green},${splotch.color.blue})`,
            overflow: 'visible',
            pointerEvents: 'none',
            transform: `rotate(${splotchRotations[index] ?? 0}deg)`,
          } satisfies CSSProperties

          return (
            <div
              className={`relative h-[90%] w-[90%] cursor-pointer transition-transform ${splotch.active ? 'scale-110' : ''}`}
              key={index}
              onClick={splotch.setActive}
            >
              {splotch.active && (
                <FaSplotch
                  className="absolute inset-0 h-full w-full"
                  style={{
                    ...style,
                    stroke: 'white',
                    strokeWidth: 50,
                  }}
                />
              )}
              <FaSplotch
                className="absolute inset-0 h-full w-full"
                style={{
                  ...style,
                  stroke: 'black',
                  strokeWidth: 15,
                }}
              />
            </div>
          )
        })}
      </div>
    </div>
  )
}
