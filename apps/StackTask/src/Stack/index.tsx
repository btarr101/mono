import { useCallback, useRef } from 'react'
import { useStackStore } from '../contexts/StackContext'
import { Endpoint } from './Endpoint'
import { ItemCard } from './ItemCard'

type Point = { x: number; y: number }

const getCenter = (boundingRect: DOMRect) => ({
  x: (boundingRect.left + boundingRect.right) / 2,
  y: (boundingRect.top + boundingRect.bottom) / 2,
})

const getDistance = (a: Point, b: Point) => Math.sqrt((a.x - b.x) ** 2 + (a.y - b.y) ** 2)

export const Stack = () => {
  const { items, push, queue, moveBefore } = useStackStore()
  const itemCardRefs = useRef(new Map<string, HTMLDivElement | null>())

  const ref = useRef<HTMLDivElement>(null)
  const onDrag = useCallback(
    (id: string) => () => {
      const element = itemCardRefs.current.get(id)
      if (!element) return

      const center = getCenter(element.getBoundingClientRect())
      const otherBoundingRects = Array.from(itemCardRefs.current.entries())
        .filter(([otherId]) => id !== otherId)
        .flatMap(([id, element]) =>
          element ? [{ id, boundingRect: element.getBoundingClientRect() }] : [],
        )

      const closestBoundingRect = otherBoundingRects.reduce(
        (current, { id, boundingRect }) => {
          const distance = getDistance(center, getCenter(boundingRect))
          return !current || current.distance > distance
            ? {
                id,
                distance,
              }
            : current
        },
        undefined as { id: string; distance: number } | undefined,
      )

      console.log(closestBoundingRect)

      if (closestBoundingRect) {
        moveBefore(id, closestBoundingRect.id)
      }
    },
    [moveBefore],
  )

  return (
    <div
      ref={ref}
      style={{
        display: 'flex',
        flexWrap: 'wrap',
        flexDirection: 'row',
        background: 'green',
        width: '100%',
        alignItems: 'center',
        justifyContent: 'start',
      }}
    >
      <Endpoint onClick={() => queue({ content: 'Queued Item' })} />
      {items.map(item => (
        <ItemCard
          ref={element => {
            itemCardRefs.current.set(item.id, element)
          }}
          key={item.id}
          item={item}
          onDragEnd={onDrag(item.id)}
        />
      ))}
      {items.length !== 0 && <Endpoint onClick={() => push({ content: 'Pushed Item' })} />}
    </div>
  )
}
