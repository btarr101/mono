import './Palette.css'
import 'react-beautiful-color/dist/react-beautiful-color.css'

import { ColorPicker } from 'react-beautiful-color'

import { usePalette } from '../../contexts/PaletteContext'

export type PaletteProps = {
  orientation?: 'vertical' | 'horizontal'
}

export const Palette = ({ orientation = 'horizontal' }: PaletteProps) => {
  // I have no idea why, but using activeSplotch here does not cause freezes,
  // while using it with the easel does
  const { activeSplotch, splotches } = usePalette()

  return (
    <div className={`flex ${orientation === 'vertical' ? 'flex-col' : ''} h-full`}>
      <ColorPicker
        className="palette-picker h-full w-full aspect-square border-3 shadow-none"
        color={
          activeSplotch
            ? {
                type: 'rgb',
                r: activeSplotch.color.red,
                g: activeSplotch.color.green,
                b: activeSplotch.color.blue,
              }
            : undefined
        }
        onChange={color => {
          const { r, g, b } = color.getRgb()
          activeSplotch?.setColor({
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
      <div className="grid h-full w-full max-w-full grid-cols-3 grid-rows-3 gap-4 p-2">
        {splotches.map((splotch, index) => (
          <div
            className={`aspect-square w-full border-2 rounded-full cursor-pointer transition-transform ${splotch.active ? 'scale-110 outline-2 outline-white' : ''}`}
            key={index}
            style={{
              backgroundColor: `rgb(${splotch.color.red},${splotch.color.green},${splotch.color.blue})`,
            }}
            onClick={splotch.setActive}
          />
        ))}
      </div>
    </div>
  )
}
