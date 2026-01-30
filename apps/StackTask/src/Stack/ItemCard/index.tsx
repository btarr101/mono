import type { PointerEvent } from 'react'
import { animate, motion, useMotionValue, usePresence } from 'motion/react'
import type { StackItem } from '../../contexts/StackContext/types'
import { useEffect, useRef, useState } from 'react'
import { flushSync } from 'react-dom'
import { boxShadow } from '../../util/css'
import { cardHeight, cardRadius, cardWidth } from '../../style'
import { DropPoint } from './DropPoint'
import swish from '../../assets/sounds/swish.wav'

export type ItemCardProps = {
  item: StackItem
  onDragEnd?: () => void
}

export const ItemCard = ({ item: { id, content, color }, onDragEnd }: ItemCardProps) => {
  const [isPresent, safeToRemove] = usePresence()
  const [dragOrigin, setDragOrigin] = useState<{ x: number; y: number } | null>(null)
  const ref = useRef<HTMLDivElement | null>(null)
  const x = useMotionValue(0)
  const y = useMotionValue(0)
  const [zIndex, setZIndex] = useState(0)
  const scale = useMotionValue(1)

  const dragging = dragOrigin !== null

  useEffect(() => {
    if (!isPresent) {
      const audio = new Audio(swish)
      audio.volume = 0.3
      audio.play().catch()
      safeToRemove()
    }
  }, [isPresent, safeToRemove])

  const handlePointerDown = (event: PointerEvent<HTMLDivElement>) => {
    if (event.button !== 0) return

    x.stop()
    y.stop()
    scale.stop()
    setDragOrigin({ x: event.clientX - x.get(), y: event.clientY - y.get() })
    setZIndex(999)
    animate(scale, 1.5)
    event.currentTarget.setPointerCapture(event.pointerId)
  }

  const handlePointerMove = (event: PointerEvent<HTMLDivElement>) => {
    if (!dragOrigin) return

    x.set(event.clientX - dragOrigin.x)
    y.set(event.clientY - dragOrigin.y)
  }

  const handlePointerUp = (event: PointerEvent<HTMLDivElement>) => {
    if (event.button !== 0) return

    const element = ref.current!

    const before = element.getBoundingClientRect()
    flushSync(() => onDragEnd?.())
    const after = element.getBoundingClientRect()

    const dx = after.x - before.x
    const dy = after.y - before.y

    x.set(x.get() - dx)
    y.set(y.get() - dy)

    animate(x, 0)
    animate(y, 0)
    animate(scale, 1).then(() => setZIndex(0))

    setDragOrigin(null)
  }

  return (
    <div
      style={{
        zIndex,
        display: 'flex',
      }}
    >
      <DropPoint beforeId={id} />
      <motion.div
        animate={{
          scale: 1,
          x: 0,
          y: 0,
          boxShadow: dragging
            ? boxShadow({
                x: '0px',
                y: '16px',
                blur: '24px',
              })
            : boxShadow({
                x: '0px',
                y: '6px',
                blur: '12px',
              }),
        }}
        exit={{
          scale: 0,
          opacity: 1,
          y: 0,
        }}
        id={id}
        initial={{ scale: 0, y: 50 }}
        layout={!dragging}
        ref={ref}
        style={{
          fontSize: '12px',
          overflowWrap: 'anywhere',
          x,
          y,
          width: cardWidth,
          height: cardHeight,
          backgroundColor: color,
          borderRadius: cardRadius,
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          cursor: 'grab',
          userSelect: 'none',
          scale,
          touchAction: 'none',
        }}
        onPointerDown={handlePointerDown}
        onPointerMove={handlePointerMove}
        onPointerUp={handlePointerUp}
      >
        <span
          style={{
            margin: 10,
          }}
        >
          {content}
        </span>
      </motion.div>
      <DropPoint afterId={id} />
    </div>
  )
}
