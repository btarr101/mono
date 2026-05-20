import 'konva/lib/shapes/Circle'

import Image from '@son426/vite-image/react'
import type { Vector2d } from 'konva/lib/types'
import { memo, useCallback, useMemo, useRef, useState } from 'react'
import { Circle, Layer, Stage } from 'react-konva/lib/ReactKonvaCore'

import { EASEL_IMAGE } from '../../constants'
import { useEasel } from '../../contexts/EaselContext'
import { usePaletteActiveSplotch } from '../../contexts/PaletteContext'
import { usePointerPrimaryUpdated } from '../../contexts/PointerContext'
import type { Color, Position } from '../../types'
import { divideVectors, getPositionsInStroke } from './util'

export type EaselProps = {
  stageSize?: {
    width: number
    height: number
    scale: number
  }
}

const pad = 32
const positionsAndColors = Array.from({ length: 24 }).flatMap((_, row) =>
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
const positions = positionsAndColors.map(({ x, y }) => ({ x, y }))

export const Easel = ({ stageSize }: EaselProps) => {
  const { activeSplotch } = usePaletteActiveSplotch()
  const { leds, setLed } = useEasel()
  const stageContainerRef = useRef<HTMLDivElement | null>(null)

  // Scaling
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

  // Drawing
  const previousPointerStagePositionRef = useRef<Vector2d | null>(null)

  // State used for visuals ONLY
  const [pointerStagePosition, setPointerStagePosition] = useState<Vector2d | null>(null)

  const handlePaint = useCallback(
    (primaryDown: boolean, position: Position | null) => {
      if (!position) return

      const containerRect = stageContainerRef.current?.getBoundingClientRect()
      if (!containerRect) return

      const stagePosition = divideVectors(
        {
          x: position.x - containerRect.left,
          y: position.y - containerRect.top,
        },
        stageScale ?? { x: 1, y: 1 },
      )

      setPointerStagePosition(stagePosition)

      if (!primaryDown) {
        previousPointerStagePositionRef.current = null
        return
      }

      if (!activeSplotch?.color) return

      const previousPointerPosition = previousPointerStagePositionRef.current
      previousPointerStagePositionRef.current = stagePosition

      getPositionsInStroke(
        positions,
        previousPointerPosition ?? stagePosition,
        stagePosition,
      ).forEach(index => setLed(index, activeSplotch.color))
    },
    [activeSplotch, setLed, stageScale],
  )

  usePointerPrimaryUpdated(handlePaint)

  const ledGlows = useMemo(
    () =>
      leds.map(({ color }, index) => {
        const position = positionsAndColors[index]
        if (!position) return null

        return <LedGlow color={color} key={index} x={position.x} y={position.y} />
      }),
    [leds],
  )

  return (
    <div className="flex h-full w-full min-h-0 min-w-0 items-center justify-center pointer-events-none">
      {stageSize ? (
        <div
          className="relative overflow-clip rounded-2xl shadow-2xl"
          ref={stageContainerRef}
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
              {ledGlows}
              {pointerStagePosition && (
                <Circle
                  radius={52}
                  stroke="black"
                  strokeWidth={4}
                  x={pointerStagePosition.x}
                  y={pointerStagePosition.y}
                />
              )}
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
}

const LedGlow = memo(({ color, x, y }: LedGlowProps) => {
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
      listening={false}
      radius={52}
      x={x}
      y={y}
    />
  )
})

LedGlow.displayName = 'LedGlow'
