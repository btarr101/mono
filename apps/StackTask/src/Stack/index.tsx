import { useCallback } from 'react'
import { useStackStore } from '../contexts/StackContext'
import { Endpoint } from './Endpoint'
import { ItemCard } from './ItemCard'
import { getCenter, getDistance, type Position } from './util'
import { choose } from '../util/arrays'
import { cardWidth, taskColors } from '../style'
import { DropPoint } from './DropPoint'

export const Stack = () => {
  const { items, push, queue, moveBefore } = useStackStore()

  const onDragEnd = useCallback(
    (id: string) => () => {
      const element = document.getElementById(id)
      if (!(element instanceof HTMLDivElement)) return

      const center = getCenter(element.getBoundingClientRect())
      const closestDropPointId = getClosestDropPointBeforeIdFrom(center)

      if (closestDropPointId !== undefined) {
        moveBefore(id, closestDropPointId)
      }
    },
    [moveBefore],
  )

  return (
    <div
      style={{
        border: '1',
        borderColor: 'black',
        borderStyle: 'solid',
        display: 'grid',
        gridTemplateColumns: `repeat(auto-fill, ${cardWidth})`,
        gridAutoRows: 'auto',
        gap: '10px',

        paddingLeft: '10%',
        paddingRight: '10%',
        marginBottom: '10vh',
      }}
    >
      <Endpoint onClick={() => queue({ content: 'Queued Item', color: choose(taskColors) })} />
      {items.map(item => (
        <ItemCard item={item} key={item.id} onDragEnd={onDragEnd(item.id)} />
      ))}
      {items.length !== 0 && (
        <div
          style={{
            display: 'flex',
          }}
        >
          <DropPoint />
          <Endpoint onClick={() => push({ content: 'Pushed Item', color: choose(taskColors) })} />
        </div>
      )}
    </div>
  )
}

type DropPointRect = {
  beforeId: string | null
  boundingRect: DOMRect
}

const getDropPointRects = () => {
  const dropPoints: NodeListOf<HTMLDivElement> = document.querySelectorAll(
    'div[x-drop-point-before]',
  )

  return Array.from(dropPoints.values()).flatMap(element => {
    const rawBeforeId = element.getAttribute('x-drop-point-before')
    if (rawBeforeId === null) return []
    const beforeId = rawBeforeId === 'NONE' ? null : rawBeforeId

    return [
      {
        beforeId,
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
  return dropPointRects.reduce(
    (current, { beforeId, boundingRect }) => {
      const distance = getDistance(position, getCenter(boundingRect))
      return !current || current.distance > distance
        ? {
            beforeId,
            distance,
          }
        : current
    },
    undefined as { beforeId: string | null; distance: number } | undefined,
  )?.beforeId
}
