import { useCallback } from 'react'
import { useStackStore } from '../contexts/StackContext'
import { AddButton } from './AddButton'
import { ItemCard } from './ItemCard'
import { getCenter, getDistance, type Position } from './util'
import { choose } from '../util/arrays'
import { cardWidth, taskColors } from '../style'
import { AnimatePresence } from 'motion/react'
import { PopButton } from './PopButton'

export const Stack = () => {
  const { items, pop, push, moveBefore, moveAfter } = useStackStore()

  const onDragEnd = useCallback(
    (id: string) => () => {
      const element = document.getElementById(id)
      if (!(element instanceof HTMLDivElement)) return

      const center = getCenter(element.getBoundingClientRect())
      const closestDropPoint = getClosestDropPointBeforeIdFrom(center)

      if (closestDropPoint) {
        if (closestDropPoint.beforeId) {
          moveBefore(id, closestDropPoint.beforeId)
        } else {
          moveAfter(id, closestDropPoint.afterId)
        }
      }
    },
    [moveBefore, moveAfter],
  )

  return (
    <div
      style={{
        display: 'grid',
        gridTemplateColumns: `repeat(auto-fill, ${cardWidth})`,
        gridAutoRows: 'auto',
        gap: '10px',
        justifyContent: 'center',
        marginLeft: '10%',
        marginRight: '10%',
        marginTop: '5%',
        marginBottom: '5%',
      }}
    >
      <AnimatePresence>
        <PopButton onClick={pop} />

        {items.map(item => (
          <ItemCard item={item} key={item.id} onDragEnd={onDragEnd(item.id)} />
        ))}

        <AddButton key="push" onClick={content => push({ content, color: choose(taskColors) })} />
      </AnimatePresence>
    </div>
  )
}

type DropPointRect = {
  beforeId: string | null
  afterId: string | null
  boundingRect: DOMRect
}

const getDropPointRects = () => {
  const dropPoints: NodeListOf<HTMLDivElement> = document.querySelectorAll('div[x-drop-point]')

  return Array.from(dropPoints.values()).flatMap(element => {
    const beforeId = element.getAttribute('x-before')
    const afterId = element.getAttribute('x-after')

    return [
      {
        beforeId,
        afterId,
        boundingRect: element.getBoundingClientRect(),
      } satisfies DropPointRect,
    ]
  })
}

/**
 * Returns the "before id" of the closest drop point to a position.
 *
 * ### Note
 * `null` is a valid "before id", it means this drop point is
 * before "nothing", but it's still a drop point.
 */
const getClosestDropPointBeforeIdFrom = (position: Position) => {
  const dropPointRects = getDropPointRects()
  const closest = dropPointRects.reduce(
    (current, { beforeId, afterId, boundingRect }) => {
      const distance = getDistance(position, getCenter(boundingRect))
      return !current || current.distance > distance
        ? {
            beforeId,
            afterId,
            distance,
          }
        : current
    },
    undefined as { beforeId: string | null; afterId: string | null; distance: number } | undefined,
  )
  if (!closest) return undefined

  return {
    beforeId: closest.beforeId,
    afterId: closest.afterId,
  }
}
