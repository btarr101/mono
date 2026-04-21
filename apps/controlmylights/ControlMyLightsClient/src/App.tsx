import { Easel } from './components/Easel'
import { Palette } from './components/Palette'
import { COMBINED_ASPECT_DESKTOP, EASEL_IMAGE_ASPECT_STRING } from './constants'
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

export const App = () => (
  <>
    <div className="pattern-zigzag pattern-slate-500 pattern-bg-slate-600 pattern-size-8 fixed top-0 right-0 bottom-0 left-0 -z-10 min-h-screen" />
    <div className="flex h-full w-full max-w-full flex-col p-20">
      <h1 className="text-5xl text-center font-custom">Control My Lights</h1>
      <div
        className="flex flex-1 min-h-0 w-full items-center justify-center"
        style={{ containerType: 'size' }}
      >
        <div
          className="max-w-full overflow-hidden flex flex-row"
          style={{
            aspectRatio: COMBINED_ASPECT_DESKTOP,
            width: `min(100cqw,calc(100cqh*${COMBINED_ASPECT_DESKTOP}))`,
          }}
        >
          <PaletteProvider initialSplotchColors={initialSplotchColors}>
            <div className="w-min mx-auto h-full max-w-full aspect-1/2 p-4">
              <Palette />
            </div>
            <div
              className="w-full mx-auto h-full max-w-full p-4"
              style={{
                aspectRatio: EASEL_IMAGE_ASPECT_STRING,
                width: `min(100cqw,calc(100cqh*${EASEL_IMAGE_ASPECT_STRING}))`,
              }}
            >
              <EaselProvider initialLedAttributes={initialLedAttributes}>
                <Easel />
              </EaselProvider>
            </div>
          </PaletteProvider>
        </div>
      </div>
    </div>
  </>
)
