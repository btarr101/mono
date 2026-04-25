import {
  type ComponentPropsWithoutRef,
  type PropsWithChildren,
  useLayoutEffect,
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
    scale: 0,
  })

  useLayoutEffect(() => {
    const onResize = () => {
      if (!container.current) return
      const containerWidth = container.current.offsetWidth
      const containerHeight = container.current.offsetHeight
      const scale = Math.min(containerWidth / sceneWidth, containerHeight / sceneHeight)

      setStageSize({
        width: sceneWidth * scale,
        height: sceneHeight * scale,
        scale,
      })
    }

    const element = container.current
    if (!element) return

    onResize()
    const observer = new ResizeObserver(onResize)
    observer.observe(element)
    return () => observer.disconnect()
  }, [sceneHeight, sceneWidth])

  const { style: divStyle, ...restDivProps } = divProps ?? {}

  return (
    <div
      ref={container}
      style={{ alignItems: 'center', display: 'flex', justifyContent: 'center', ...divStyle }}
      {...restDivProps}
    >
      <Stage
        height={stageSize.height}
        scaleX={stageSize.scale}
        scaleY={stageSize.scale}
        width={stageSize.width}
        {...stageProps}
      >
        {children}
      </Stage>
    </div>
  )
}
