import { motion, type MotionStyle } from 'motion/react'
import { cardHeight, cardRadius, cardWidth, colors } from '../style'

export type EndpointProps = {
  onClick?: React.MouseEventHandler<HTMLDivElement> | undefined
}

export const Endpoint = ({ onClick }: EndpointProps) => (
  <motion.div
    layout
    style={outerStyle}
    whileHover={{ scale: 1.1 }}
    whileTap={{ scale: 0.9 }}
    onClick={onClick}
  >
    <div style={innerStyle}></div>
  </motion.div>
)

const outerStyle = {
  width: cardWidth,
  height: cardHeight,

  borderRadius: cardRadius,
  display: 'flex',
  alignContent: 'stretch',
  justifyContent: 'stretch',
} satisfies MotionStyle

const innerStyle = {
  outline: `4px dashed ${colors.nearBlack}`,
  width: '100%',
  borderRadius: cardRadius,
  margin: 10,
} satisfies MotionStyle
