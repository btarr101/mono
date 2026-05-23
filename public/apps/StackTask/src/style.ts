export const colors = {
  nearBlack: '#00000d',
  charcoal: '#252928',
  mossGray: '#566349',
  khakiGray: '#afb38b',
  parchment: '#d2d4ab',
  ivory: 'rgba(242, 241, 233, 1)',
  wine: '#750641',
  crimson: '#d60f37',
  rose: '#eb546b',
  burntOrange: '#cf5b13',
  amber: '#e6ab22',
  honey: '#f5c86e',
  forest: '#118a19',
  neonGreen: '#1fcf62',
  mint: '#6ff2c2',
  royal: '#0d40a6',
  sky: '#167cb8',
  ice: '#7bcde8',
  indigo: '#2d0a8c',
  violet: '#6a18ad',
  orchid: '#e37df5',
  magenta: '#990e53',
  fuchsia: '#cf1992',
  bubblegum: '#f065dd',
} as const

export const taskColors = [
  colors.parchment,
  colors.rose,
  colors.burntOrange,
  colors.amber,
  colors.honey,
  colors.neonGreen,
  colors.mint,
  colors.sky,
  colors.ice,
  colors.orchid,
  colors.bubblegum,
] as const

export const cardWidth = '100px'
export const cardHeight = '100px'
export const cardRadius = 5
export const cardMargin = 4
