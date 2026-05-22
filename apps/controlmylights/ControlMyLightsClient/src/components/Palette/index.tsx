import './Palette.css'
import 'react-beautiful-color/dist/react-beautiful-color.css'

import { ColorPicker } from 'react-beautiful-color'
import { FaPaintbrush } from 'react-icons/fa6'

import {
  usePaletteActions,
  usePaletteActiveSplotch,
  usePaletteBrushScale,
  usePaletteSplotches,
} from '../../contexts/PaletteContext'

export type PaletteProps = {
  orientation?: 'vertical' | 'horizontal'
}

export const Palette = ({ orientation = 'horizontal' }: PaletteProps) => {
  const splotches = usePaletteSplotches()
  const { activeSplotchIndex } = usePaletteActiveSplotch()
  const { setActiveSpotchIndex, setBrushScale } = usePaletteActions()
  const brushScale = usePaletteBrushScale()

  return (
    <div
      className={`flex min-h-0 min-w-0 items-center ${orientation === 'vertical' ? 'h-full flex-col' : 'w-full flex-row'}`}
    >
      <div className="flex-1 aspect-square">
        <MyColorPicker />
      </div>

      {/*
      This component is a lil' fucked - all of the sizing is driven by the component below.

      This is why the lack of a gap between the saturation and the splotches is a "styalistic choice"
      */}
      <div
        className={`aspect-square ${orientation === 'vertical' ? 'h-1/2 w-auto' : 'w-1/2 h-auto'}`}
      >
        <div className="h-full mx-auto grid grid-rows-[minmax(0,1fr)_auto] m-auto w-fit">
          <div className="h-full aspect-square grid grid-cols-3 grid-rows-3 gap-3 mt-2 mx-auto">
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
          </div>
          <div className="w-full mx-auto items-center flex mt-4 -mb-2">
            <FaPaintbrush className="mr-2 border-2 rounded-full p-1 h-8 min-w-fit min-h-fit aspect-square" />
            <input
              className="appearance-none w-full bg-black h-4 shadow-lg rounded-2xl
                        [&::-webkit-slider-thumb]:cursor-grab
                        [&::-webkit-slider-thumb]:appearance-none 
                        [&::-webkit-slider-thumb]:h-7 
                        [&::-webkit-slider-thumb]:w-7
                        [&::-webkit-slider-thumb]:border-2
                      [&::-webkit-slider-thumb]:bg-white 
                        [&::-webkit-slider-thumb]:rounded-full"
              max={100}
              min={0}
              type="range"
              value={brushScale}
              onChange={event => setBrushScale(parseInt(event.target.value))}
            />
          </div>
        </div>
      </div>
    </div>
  )
}

const MyColorPicker = () => {
  const { activeSplotchIndex, activeSplotch } = usePaletteActiveSplotch()
  const { setSplotchColor } = usePaletteActions()

  return (
    <ColorPicker
      className="palette-picker shadow-none grid grid-rows-[minmax(0,1fr)_auto] h-full m-auto w-fit"
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
      <ColorPicker.Saturation className="aspect-square w-fit mx-auto" />
      <div className="w-full mt-2">
        <ColorPicker.Hue className="h-4 w-full" />
      </div>
    </ColorPicker>
  )
}
