import { animate, motion, useMotionValue } from 'motion/react'
import type { StackItem } from '../../contexts/StackContext/types'
import { useRef, useState } from 'react'
import { flushSync } from 'react-dom'
import { boxShadow } from '../../util/css'

export type ItemCardProps = {
  item: StackItem
  onDragEnd?: () => void
}

export const ItemCard = ({ item: { id, content, color }, onDragEnd }: ItemCardProps) => {
  const [dragOrigin, setDragOrigin] = useState<{ x: number; y: number } | null>(null)
  const ref = useRef<HTMLDivElement | null>(null)
  const x = useMotionValue(0)
  const y = useMotionValue(0)

  const dragging = dragOrigin !== null

  return (
    <motion.div
      ref={ref}
      id={id}
      layout={!dragging}
      onPointerDown={event => {
        x.stop()
        y.stop()
        setDragOrigin({ x: event.clientX - x.get(), y: event.clientY - y.get() })
        event.currentTarget.setPointerCapture(event.pointerId)
      }}
      onPointerMove={event => {
        if (!dragOrigin) return

        x.set(event.clientX - dragOrigin.x)
        y.set(event.clientY - dragOrigin.y)
      }}
      onPointerUp={() => {
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

        setDragOrigin(null)
      }}
      initial={{ scale: 0 }}
      animate={{
        scale: 1,
        boxShadow: boxShadow({
          x: '0px',
          y: '6px',
          blur: '12px',
        }),
        ...(dragging
          ? {
              boxShadow: boxShadow({
                x: '0px',
                y: '16px',
                blur: '24px',
              }),
              scale: 1.25,
              zIndex: 999,
            }
          : {
              zIndex: 0,
            }),
      }}
      style={{
        x,
        y,
        width: 100,
        height: 100,
        backgroundColor: color,
        borderRadius: 5,
        // marginLeft: 4,
        // marginBottom: 4,
        margin: 4,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        cursor: 'grab',
        userSelect: 'none',
      }}
    >
      <span>{content}</span>
    </motion.div>
  )
}
