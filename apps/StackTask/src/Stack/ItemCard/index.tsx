import { type Ref } from 'react'
import { motion, useAnimation, type MotionStyle, type PanInfo } from 'motion/react'
import type { StackItem } from '../../contexts/StackContext/types'

type DragHandler = (event: MouseEvent | TouchEvent | PointerEvent, info: PanInfo) => void

export type ItemCardProps = {
  ref?: Ref<HTMLDivElement>
  item: StackItem
  onDrag?: DragHandler
  onDragStart?: DragHandler
  onDragEnd?: DragHandler
  isDragging?: boolean
}

export const ItemCard = ({
  ref,
  item: { content },
  onDrag,
  onDragStart,
  onDragEnd,
  isDragging = false,
}: ItemCardProps) => {
  const controls = useAnimation()

  return (
    <motion.div
      layout
      ref={ref}
      animate={controls}
      whileHover={{ scale: 1.03 }}
      whileTap={{ scale: 0.97 }}
      // dragging
      drag
      whileDrag={{ scale: 1.05, zIndex: 20, boxShadow: '0 15px 30px rgba(15, 23, 42, 0.25)' }}
      onDrag={onDrag}
      onDragStart={onDragStart}
      onDragEnd={(event, info) => {
        if (onDragEnd) {
          onDragEnd(event, info)
        }
      }}
      dragElastic={0.12}
      dragMomentum={false}
      style={{
        ...style,
        cursor: 'grab',
        userSelect: 'none',
        boxShadow: isDragging
          ? '0 12px 24px rgba(15, 23, 42, 0.2)'
          : '0 2px 6px rgba(15, 23, 42, 0.15)',
      }}
    >
      <span>{content}</span>
    </motion.div>
  )
}

const style = {
  width: 100,
  height: 100,
  backgroundColor: '#09f',
  borderRadius: 5,
  margin: 4,
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
} satisfies MotionStyle
