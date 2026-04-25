import Image from '@son426/vite-image/react'
import type { KonvaEventObject } from 'konva/lib/Node'
import type { Vector2d } from 'konva/lib/types'
import { useMemo } from 'react'
import { Circle, Layer, Stage } from 'react-konva'
import useMeasure from 'react-use-measure'

import { EASEL_IMAGE } from '../../constants'
import { useEasel } from '../../contexts/EaselContext'
import { usePalette } from '../../contexts/PaletteContext'
import type { Color } from '../../types'

const pad = 32
const positions = Array.from({ length: 24 }).flatMap((_, row) =>
  Array.from({ length: 48 }).map((_, col) => ({
    color: {
      red: Math.random() * 255,
      green: Math.random() * 255,
      blue: Math.random() * 255,
    },
    x: pad + ((EASEL_IMAGE.width - pad) / 48) * col,
    y: pad + ((EASEL_IMAGE.height - pad) / 24) * row,
  })),
)

export const Easel = () => {
  const { splotches } = usePalette()
  const activeSplotch = splotches.find(({ active }) => active) ?? null
  const { leds } = useEasel()

  const [ref, bounds] = useMeasure({
    scroll: false,
  })

  const scale = useMemo(
    () =>
      bounds.width !== null && bounds.height !== null
        ? ({
            x: bounds.width / EASEL_IMAGE.width,
            y: bounds.height / EASEL_IMAGE.height,
          } satisfies Vector2d)
        : undefined,
    [bounds.height, bounds.width],
  )

  return (
    // overflow-clip is just to prevent the background image from clipping over the rounded borders
    <div className="relative w-full h-full border-3 rounded-md overflow-clip" ref={ref}>
      <Image draggable={false} fill={true} placeholder="blur" priority={true} src={EASEL_IMAGE} />
      {/* TODO: decide whether to keep dynamic scale vs fixed stage sizing */}
      <Stage
        className="absolute left-0 right-0 top-0 bottom-0"
        height={EASEL_IMAGE.height}
        scale={scale}
        width={EASEL_IMAGE.width}
      >
        <Layer>
          {leds.map(({ color, setColor }, index) => {
            const position = positions[index]
            if (!position) return null

            return (
              <LedGlow
                color={color}
                key={index}
                x={position.x}
                y={position.y}
                onPointerMove={event => {
                  if (event.evt.buttons > 0 && activeSplotch) {
                    setColor(activeSplotch.color)
                  }
                }}
              />
            )
          })}
        </Layer>
      </Stage>
    </div>
  )
}

export type LedGlowProps = {
  color: Color
  x: number
  y: number
  onPointerMove?: (event: KonvaEventObject<PointerEvent>) => void
}

const LedGlow = ({ color, x, y, onPointerMove }: LedGlowProps) => {
  const brightness = Math.pow((color.red + color.green + color.blue) / (255 * 3), 0.5)
  return (
    <Circle
      fillRadialGradientColorStops={[
        0,
        `rgba(${color.red}, ${color.green}, ${color.blue}, ${brightness})`,
        1,
        `rgba(${color.red}, ${color.green}, ${color.blue}, 0)`,
      ]}
      fillRadialGradientEndPoint={{ x: 0, y: 0 }}
      fillRadialGradientEndRadius={52}
      fillRadialGradientStartPoint={{ x: 0, y: 0 }}
      fillRadialGradientStartRadius={0}
      listening={!!onPointerMove}
      radius={52}
      x={x}
      y={y}
      onPointerMove={onPointerMove}
    />
  )
}
