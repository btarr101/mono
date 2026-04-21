import {
  type ComponentPropsWithoutRef,
  type PropsWithChildren,
  useEffect,
  useRef,
  useState,
} from 'react'
import { Stage, type StageProps } from 'react-konva'

type ResponsiveStageProps = PropsWithChildren<{
  sceneWidth: number
  sceneHeight: number
  divProps?: Omit<ComponentPropsWithoutRef<'div'>, 'children'>
  stageProps?: Omit<StageProps, 'children' | 'width' | 'height'>
}>

export const ResponsiveStage = ({
  sceneWidth,
  sceneHeight,
  divProps,
  stageProps,
  children,
}: ResponsiveStageProps) => {
  const container = useRef<HTMLDivElement | null>(null)

  const [stageSize, setStageSize] = useState({
    width: 0,
    height: 0,
    scaleX: 0,
    scaleY: 0,
  })

  useEffect(() => {
    const onResize = () => {
      if (!container.current) return
      const containerWidth = container.current.offsetWidth
      const containerHeight = container.current.offsetHeight
      setStageSize({
        width: containerWidth,
        height: containerHeight,
        scaleX: containerWidth / sceneWidth,
        scaleY: containerHeight / sceneHeight,
      })
    }

    const element = container.current
    if (!element) return

    onResize()
    const observer = new ResizeObserver(onResize)
    observer.observe(element)
    return () => observer.disconnect()
  }, [sceneHeight, sceneWidth])

  return (
    <div ref={container} {...divProps}>
      <Stage
        height={stageSize.height}
        scaleX={stageSize.scaleX}
        scaleY={stageSize.scaleY}
        width={stageSize.width}
        {...stageProps}
      >
        {children}
      </Stage>
    </div>
  )
}
