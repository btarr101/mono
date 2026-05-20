import './Palette.css'
import 'react-beautiful-color/dist/react-beautiful-color.css'

import { ColorPicker } from 'react-beautiful-color'

import {
  usePaletteActions,
  usePaletteActiveSplotch,
  usePaletteSplotches,
} from '../../contexts/PaletteContext'

export type PaletteProps = {
  orientation?: 'vertical' | 'horizontal'
}

export const Palette = ({ orientation = 'horizontal' }: PaletteProps) => {
  const splotches = usePaletteSplotches()
  const { activeSplotchIndex, activeSplotch } = usePaletteActiveSplotch()
  const { setSplotchColor, setActiveSpotchIndex } = usePaletteActions()

  return (
    <div
      className={`flex min-h-0 min-w-0 items-center gap-4 ${orientation === 'vertical' ? 'h-full flex-col' : 'w-full flex-row'}`}
    >
      <div
        className={`min-h-0 min-w-0 ${orientation === 'vertical' ? 'h-1/2 w-fit' : 'w-1/2 aspect-square'}`}
      >
        <ColorPicker
          className={`aspect-square palette-picker shadow-none grid w-full grid-rows-[minmax(0,1fr)_auto] ${orientation === 'vertical' ? 'max-h-full h-full' : 'max-w-full h-full w-full'}`}
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
            if (activeSplotchIndex === undefined) return

            const { r: red, g: green, b: blue } = color.getRgb()
            setSplotchColor(activeSplotchIndex, { red, green, blue })
          }}
        >
          <ColorPicker.Saturation
            className={`rounded-b-lg ${orientation === 'vertical' ? 'h-full min-h-0' : 'h-full w-full min-h-0 min-w-0'}`}
          />
          <div className="w-full py-2">
            <ColorPicker.Hue className="h-4 w-full" />
          </div>
        </ColorPicker>
      </div>
      <div
        className={`aspect-square min-h-0 min-w-0 grid grid-cols-3 grid-rows-3 gap-3 ${orientation === 'vertical' ? 'h-1/2 w-auto' : 'w-1/2 h-auto'}`}
      >
        {splotches.map((splotch, index) => (
          <div
            className={`aspect-square border-5 border-white shadow-lg rounded-full cursor-pointer transition-transform ${index === activeSplotchIndex ? 'scale-110 outline-2 outline-black' : ''}`}
            key={index}
            style={{
              backgroundColor: `rgb(${splotch.color.red},${splotch.color.green},${splotch.color.blue})`,
            }}
            onClick={() => setActiveSpotchIndex(index)}
          />
        ))}
        {/* TODO SLIDER HERE TO SELECT BRUSH SIZE */}
      </div>
    </div>
  )
}
