import 'konva/lib/shapes/Circle'

import Image from '@son426/vite-image/react'
import type { KonvaEventObject } from 'konva/lib/Node'
import type { Vector2d } from 'konva/lib/types'
import { useMemo } from 'react'
import { Circle, Layer, Stage } from 'react-konva/lib/ReactKonvaCore'

import { EASEL_IMAGE } from '../../constants'
import { useEasel } from '../../contexts/EaselContext'
import { usePaletteActiveSplotch } from '../../contexts/PaletteContext'
import type { Color } from '../../types'

export type EaselProps = {
  stageSize?: {
    width: number
    height: number
    scale: number
  }
}

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

export const Easel = ({ stageSize }: EaselProps) => {
  const { activeSplotch } = usePaletteActiveSplotch()
  const { leds, setLed } = useEasel()

  const stageScale = useMemo(
    () =>
      stageSize
        ? ({
            x: stageSize.scale,
            y: stageSize.scale,
          } satisfies Vector2d)
        : undefined,
    [stageSize],
  )

  return (
    <div className="flex h-full w-full min-h-0 min-w-0 items-center justify-center">
      {stageSize ? (
        <div
          className="relative overflow-clip rounded-2xl shadow-2xl"
          style={{
            height: stageSize.height,
            width: stageSize.width,
          }}
        >
          <Image
            draggable={false}
            fill={true}
            placeholder="blur"
            priority={true}
            src={EASEL_IMAGE}
          />
          <Stage
            className="absolute left-0 right-0 top-0 bottom-0"
            height={EASEL_IMAGE.height}
            scale={stageScale}
            width={EASEL_IMAGE.width}
          >
            <Layer>
              {leds.map(({ color }, index) => {
                const position = positions[index]
                if (!position) return null

                return (
                  <LedGlow
                    color={color}
                    key={index}
                    x={position.x}
                    y={position.y}
                    onPointerMove={event => {
                      if (!activeSplotch) return
                      if (!event.evt.buttons) return

                      setLed(index, activeSplotch.color)
                    }}
                  />
                )
              })}
            </Layer>
          </Stage>
        </div>
      ) : null}
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
