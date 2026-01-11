import { motion, type MotionStyle } from 'motion/react'

export type EndpointProps = {
  onClick?: React.MouseEventHandler<HTMLDivElement> | undefined
}

const outerStyle = {
  width: 100,
  height: 100,
  borderRadius: 5,
  display: 'flex',
  alignContent: 'stretch',
  justifyContent: 'stretch',
} satisfies MotionStyle

const innerStyle = {
  outline: '4px dashed #363636ff',
  width: '100%',
  borderRadius: 5,
  margin: 10,
} satisfies MotionStyle

export const Endpoint = ({ onClick }: EndpointProps) => (
  <motion.div
    layout
    onClick={onClick}
    whileHover={{ scale: 1.1 }}
    whileTap={{ scale: 0.9 }}
    style={outerStyle}
  >
    <div style={innerStyle}></div>
  </motion.div>
)
