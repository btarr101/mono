type Length = `${number}px` | `${number}rem` | `${number}em` | 0
type Hex = `#${string}`
type RGB = `rgb(${number},${number},${number})` | `rgb(${number}, ${number}, ${number})`
type RGBA =
  | `rgba(${number},${number},${number},${number})`
  | `rgba(${number}, ${number}, ${number}, ${number})`
type HSL = `hsl(${number},${number}%,${number}%)` | `hsl(${number}, ${number}%, ${number}%)`
type HSLA =
  | `hsla(${number},${number}%,${number}%,${number})`
  | `hsla(${number}, ${number}%, ${number}%, ${number})`
type NamedColor =
  | 'black'
  | 'white'
  | 'red'
  | 'green'
  | 'blue'
  | 'yellow'
  | 'cyan'
  | 'magenta'
  | 'transparent'
  | 'currentColor'
export type CssColor = Hex | RGB | RGBA | HSL | HSLA | NamedColor

type BoxShadowSpec = {
  inset?: boolean
  x: Length
  y: Length
  blur?: Length
  spread?: Length
  color?: CssColor
}

export const boxShadow = ({
  inset,
  x,
  y,
  blur = 0,
  spread = 0,
  color = 'rgba(0, 0, 0, 0.25)',
}: BoxShadowSpec) => [...(inset ? ['inset'] : []), x, y, blur ?? 0, spread ?? 0, color].join(' ')
