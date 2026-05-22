import 'konva/lib/shapes/Circle'

import Image from '@son426/vite-image/react'
import type { Vector2d } from 'konva/lib/types'
import { memo, useCallback, useMemo, useRef, useState } from 'react'
import { Circle, Layer, Stage } from 'react-konva/lib/ReactKonvaCore'

import { EASEL_IMAGE } from '../../constants'
import { useEasel } from '../../contexts/EaselContext'
import { usePaletteActiveSplotch, usePaletteBrushScale } from '../../contexts/PaletteContext'
import { usePointerPrimaryUpdated } from '../../contexts/PointerContext'
import type { Color, Position } from '../../types'
import { LED_HITBOX_RADIUS, LED_RADIUS, POSITIONS } from './config'
import { divideVectors, getPositionsInStroke, lerp } from './util'

export type EaselProps = {
  stageSize?: {
    width: number
    height: number
    scale: number
  }
}

export const Easel = ({ stageSize }: EaselProps) => {
  const { activeSplotch } = usePaletteActiveSplotch()
  const { leds, setLed } = useEasel()
  const brushScale = usePaletteBrushScale()
  const brushRadius = lerp(LED_RADIUS / 4, LED_RADIUS * 2, brushScale / 100)

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
        POSITIONS,
        previousPointerPosition ?? stagePosition,
        stagePosition,
        brushRadius + LED_HITBOX_RADIUS,
      ).forEach(index => setLed(index, activeSplotch.color))
    },
    [activeSplotch, setLed, stageScale, brushRadius],
  )

  usePointerPrimaryUpdated(handlePaint)

  const ledGlows = useMemo(
    () =>
      leds.map(({ color }, index) => {
        const position = POSITIONS[index]
        if (!position) return null

        return <LedGlow color={color} key={index} x={position.x} y={position.y} />
      }),
    [leds],
  )

  return (
    <div className="items-center justify-center  select-none overscroll-none touch-none">
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
                  radius={brushRadius}
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
    <>
      {/* Uncomment below for debug */}
      {/* <Circle radius={LED_HITBOX_RADIUS} stroke="black" strokeWidth={4} x={x} y={y} /> */}
      <Circle
        fillRadialGradientColorStops={[
          0,
          `rgba(${color.red}, ${color.green}, ${color.blue}, ${brightness})`,
          1,
          `rgba(${color.red}, ${color.green}, ${color.blue}, 0)`,
        ]}
        fillRadialGradientEndPoint={{ x: 0, y: 0 }}
        fillRadialGradientEndRadius={LED_RADIUS}
        fillRadialGradientStartPoint={{ x: 0, y: 0 }}
        fillRadialGradientStartRadius={0}
        listening={false}
        radius={LED_RADIUS}
        x={x}
        y={y}
      />
    </>
  )
})

LedGlow.displayName = 'LedGlow'
