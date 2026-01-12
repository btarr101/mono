import { Fragment, useCallback } from 'react'
import { useStackStore } from '../contexts/StackContext'
import { Endpoint } from './Endpoint'
import { ItemCard } from './ItemCard'
import { getCenter, getDistance, type Position } from './util'
import { choose } from '../util/arrays'
import { taskColors } from '../style'

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
        display: 'flex',
        flexWrap: 'wrap',
        flexDirection: 'row',
        margin: '10%',
        // width: '100%',\
        width: 'fit-content',
        border: 'solid',
        borderWidth: 2,
        alignItems: 'center',
        justifyContent: 'start',
      }}
    >
      <Endpoint onClick={() => queue({ content: 'Queued Item', color: choose(taskColors) })} />
      {items.map(item => (
        <Fragment key={item.id}>
          <DropPoint beforeId={item.id} />
          <ItemCard key={item.id} item={item} onDragEnd={onDragEnd(item.id)} />
        </Fragment>
      ))}
      {items.length !== 0 && (
        <>
          <DropPoint />
          <Endpoint onClick={() => push({ content: 'Pushed Item', color: choose(taskColors) })} />
        </>
      )}
    </div>
  )
}

type DropPointProps = {
  beforeId?: string
}

const DropPoint = ({ beforeId: itemId }: DropPointProps) => (
  <div x-drop-point-before={itemId ?? 'NONE'} />
)

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
