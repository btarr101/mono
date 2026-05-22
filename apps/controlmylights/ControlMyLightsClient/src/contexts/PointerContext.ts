import { createElement, type PropsWithChildren, useLayoutEffect } from 'react'
import { useStore } from 'zustand'

import type { Position } from '../types'
import { buildContext } from '../util/buildContext'
import { buildUseStoreState, buildUseStoreSubscribe } from '../util/buildContext/util'

export type PointerStore = {
  primaryDown: boolean
  setPrimaryDown: (down: boolean) => void
  position: Position | null
  setPosition: (position: Position) => void
}

export type PointerUpdatedListener = (primaryDown: boolean, position: Position | null) => void

const { StoreProvider: PointerStoreProvider, useStoreApi: usePointerApi } =
  buildContext<PointerStore>(set => ({
    primaryDown: false,
    setPrimaryDown: primaryDown =>
      set({
        primaryDown,
      }),
    position: null,
    setPosition: position => set({ position }),
  }))

const PointerBindings = () => {
  const pointerApi = usePointerApi()
  const { setPrimaryDown, setPosition } = useStore(usePointerApi())

  useLayoutEffect(() => {
    const onPointerMove = (event: PointerEvent) =>
      pointerApi.setState({
        position: {
          x: event.clientX,
          y: event.clientY,
        },
      })
    const onPointerDown = (event: PointerEvent) =>
      pointerApi.setState({
        primaryDown: true,
        position: {
          x: event.clientX,
          y: event.clientY,
        },
      })
    const onPointerUp = (event: PointerEvent) =>
      pointerApi.setState({
        primaryDown: false,
        position: {
          x: event.clientX,
          y: event.clientY,
        },
      })
    const onPointerCancel = () => {
      setPrimaryDown(false)
    }
    const onBlur = () => setPrimaryDown(false)

    window.addEventListener('pointermove', onPointerMove, { passive: true })
    window.addEventListener('pointerdown', onPointerDown, { passive: true })
    window.addEventListener('pointerup', onPointerUp, { passive: true })
    window.addEventListener('pointercancel', onPointerCancel, { passive: true })
    window.addEventListener('blur', onBlur)

    return () => {
      window.removeEventListener('pointermove', onPointerMove)
      window.removeEventListener('pointerdown', onPointerDown)
      window.removeEventListener('pointerup', onPointerUp)
      window.removeEventListener('pointercancel', onPointerCancel)
      window.removeEventListener('blur', onBlur)
    }
  }, [pointerApi, setPosition, setPrimaryDown])

  return null
}

export const PointerProvider = ({ children }: PropsWithChildren) =>
  createElement(PointerStoreProvider, null, createElement(PointerBindings), children)

const usePointer = buildUseStoreState(usePointerApi)

const usePointerSubscribe = buildUseStoreSubscribe(usePointerApi)

const usePointerPrimaryUpdated = (listener: PointerUpdatedListener) =>
  usePointerSubscribe(({ primaryDown, position }) => {
    listener(primaryDown, position)
  })

export { usePointer, usePointerPrimaryUpdated }
