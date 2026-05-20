import { useMemo } from 'react'
import { useBreakpointCondition } from 'react-tw-breakpoints'
import useMeasure from 'react-use-measure'
import { match } from 'ts-pattern'

import { Easel } from './components/Easel'
import { LedSyncher } from './components/LedSyncher'
import { Palette } from './components/Palette'
import { EASEL_IMAGE } from './constants'
import { ApiProvider } from './contexts/ApiContext/ApiProvider'
import { EaselProvider } from './contexts/EaselContext'
import { PaletteProvider } from './contexts/PaletteContext'
import { PointerProvider } from './contexts/PointerContext'

const initialSplotchColors = Array.from({ length: 9 }).map(() => ({
  red: Math.random() * 255,
  green: Math.random() * 255,
  blue: Math.random() * 255,
}))

const initialLedColors = Array.from({ length: 24 * 48 }).map(() => ({ red: 0, green: 0, blue: 0 }))

const Background = () => (
  <>
    <div className="fixed inset-0 bg-linear-165 from-red-200/60 to-transparent -z-10" />
    <div className="fixed inset-0 bg-linear-315  from-green-200/60 to-transparent -z-10" />
    <div className="fixed inset-0 bg-linear-45 from-blue-200/60 to-transparent bg-bottom-left -z-10" />
    <div
      className="fixed -z-10 inset-0  bg-[radial-gradient(circle,rgba(0,0,0,0.08)_1.5px,transparent_1px)] 
            bg-size-[16px_16px]"
    />
  </>
)

const Header = () => (
  <div className="flex flex-row flex-10 items-center">
    <div className="flex flex-col items-end justify-center gap-3 h-full">
      <div className="w-8 h-2 bg-red-400/70 rotate-10 rounded-full" />
      <div className="w-10 h-2 bg-green-400/70 rotate-[-5deg] rounded-full" />
      <div className="w-8 h-2 bg-purple-400/70 rotate-[-20deg] rounded-full" />
    </div>
    <h1 className="max-sm:text-4xl max-md:text-5xl md:text-6xl text-center font-custom p-4 rounded-xl text-slate-950 whitespace-nowrap">
      Control My Lights
    </h1>
    <div className="flex flex-col items-start justify-center gap-3 h-full">
      <div className="w-8 h-2 bg-yellow-400/70 rotate-[-15deg] rounded-full" />
      <div className="w-10 h-2 bg-orange-400/70 rotate-[-5deg] rounded-full" />
      <div className="w-8 h-2 bg-blue-400/70 rotate-15 rounded-full" />
    </div>
  </div>
)

export const App = () => {
  const largerThanMd = useBreakpointCondition({ largerThan: 'md' })
  const [layoutRef, layoutBounds] = useMeasure({
    scroll: false,
  })

  const orientation = largerThanMd ? 'horizontal' : 'vertical'
  const easelStageSize = useMemo(() => {
    const gap = 16
    const maxHeight = layoutBounds.height
    const maxWidth = layoutBounds.width

    if (maxHeight <= 0 || maxWidth <= 0) return undefined

    const imageAspect = EASEL_IMAGE.width / EASEL_IMAGE.height
    const scale =
      match(orientation)
        .with('horizontal', () => {
          const paletteAspect = 0.5
          const stageHeight = Math.min(maxHeight, (maxWidth - gap) / (imageAspect + paletteAspect))
          if (stageHeight <= 0) return undefined

          return stageHeight / EASEL_IMAGE.height
        })
        .with('vertical', () => {
          const paletteAspect = 2
          const stageWidth = Math.min(
            maxWidth,
            (maxHeight - gap) * imageAspect * (1 / (1 + imageAspect / paletteAspect)),
          )
          if (stageWidth <= 0) return undefined

          return stageWidth / EASEL_IMAGE.width
        })
        .exhaustive() ?? 1

    return {
      width: EASEL_IMAGE.width * scale,
      height: EASEL_IMAGE.height * scale,
      scale,
    }
  }, [orientation, layoutBounds.height, layoutBounds.width])

  return (
    <PointerProvider>
      <Background />
      <div className="flex h-full w-full max-w-full flex-col p-10 items-center">
        <Header />

        <PaletteProvider initialSplotchColors={initialSplotchColors}>
          <div
            className="h-full w-full flex items-center justify-center min-h-96 min-w-64"
            ref={layoutRef}
          >
            <div
              className={`flex min-w-0 items-center gap-4 ${orientation === 'horizontal' ? 'flex-row' : 'flex-col-reverse'}`}
            >
              <div
                className="shrink-0 p-4 bg-slate-50 shadow-2xl rounded-2xl"
                style={match(orientation)
                  .returnType<React.CSSProperties>()
                  .with('horizontal', () => ({
                    height: easelStageSize?.height,
                    width: 'min-content',
                  }))
                  .with('vertical', () => ({
                    width: easelStageSize?.width,
                    height: 'min-content',
                  }))
                  .exhaustive()}
              >
                <Palette orientation={orientation === 'horizontal' ? 'vertical' : 'horizontal'} />
              </div>
              <EaselProvider initialColors={initialLedColors}>
                <Easel stageSize={easelStageSize} />
                <ApiProvider>
                  <LedSyncher />
                </ApiProvider>
              </EaselProvider>
            </div>
          </div>
        </PaletteProvider>
      </div>
    </PointerProvider>
  )
}
