import { useBreakpointCondition } from 'react-tw-breakpoints'

import { Easel } from './components/Easel'
import { Palette } from './components/Palette'
import { COMBINED_ASPECT_DESKTOP, COMBINED_ASPECT_MOBILE } from './constants'
import { EaselProvider } from './contexts/EaselContext'
import { PaletteProvider } from './contexts/PaletteContext'

const initialSplotchColors = Array.from({ length: 9 }).map(() => ({
  red: Math.random() * 255,
  green: Math.random() * 255,
  blue: Math.random() * 255,
}))

const initialLedAttributes = Array.from({ length: 24 * 48 }).map(() => ({
  color: { red: 0, green: 0, blue: 0 },
}))

const BackGround = () => (
  <div className="pattern-zigzag pattern-slate-500 pattern-bg-slate-600 pattern-size-8 fixed top-0 right-0 bottom-0 left-0 -z-10 min-h-screen" />
)

export const App = () => {
  const largerThanMd = useBreakpointCondition({ largerThan: 'md' })
  const combinedAspect = largerThanMd ? COMBINED_ASPECT_DESKTOP : COMBINED_ASPECT_MOBILE

  return (
    <>
      <BackGround />
      <div className="flex h-full w-full max-w-full flex-col p-10 items-center">
        <h1 className="text-5xl text-center font-custom border-3 p-4 rounded-xl bg-white">
          Control My Lights
        </h1>
        {/* 
      flex + flex-col + justify-center for vertical centering
      min-h-64 to prevent the child from shrinking too much
      */}
        <div
          className="h-full w-full flex flex-col justify-center min-h-64 m-4"
          style={{
            containerType: 'size', // containerType size is needed to use cqw and cqh in child
          }}
        >
          {/* mx-auto for horizontal centering */}
          <div
            className="mx-auto flex max-md:flex-col-reverse md:flex-row gap-2"
            style={{
              aspectRatio: combinedAspect,
              width: `min(100cqw,calc(100cqh*${combinedAspect}))`,
            }}
          >
            <PaletteProvider initialSplotchColors={initialSplotchColors}>
              {/* 
            w-min keeps the pallette as shrunk as possible
            aspect-1/2 keeps the pallete as a rectangle (this should switch on mobile TODO)
             */}
              <div className="md:w-min max-md:aspect-2/1 md:aspect-1/2">
                <Palette orientation={largerThanMd ? 'vertical' : 'horizontal'} />
              </div>
              <EaselProvider initialLedAttributes={initialLedAttributes}>
                <Easel />
              </EaselProvider>
            </PaletteProvider>
          </div>
        </div>
      </div>
    </>
  )
}
