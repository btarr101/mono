import { motion, type MotionStyle } from 'motion/react'
import { cardHeight, cardRadius, cardWidth, colors } from '../style'
import { useStackStore } from '../contexts/StackContext'
import buttonClick from '../assets/button-click.wav'

export type EndpointProps = {
  onClick?: React.MouseEventHandler<HTMLButtonElement> | undefined
}

export const PopButton = ({ onClick }: EndpointProps) => {
  const { items } = useStackStore()
  const active = items.length > 0

  return (
    <motion.div layout style={outerStyle}>
      <motion.div
        style={{
          position: 'relative',
          width: '100%',
          margin: 15,
          y: 5,
        }}
      >
        <motion.div
          animate={{
            position: 'absolute',
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
            borderRadius: cardRadius,
            backgroundColor: active ? colors.wine : colors.charcoal,
          }}
        />
        <motion.button
          animate={{
            position: 'absolute',
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
            background: 'red',
            borderRadius: cardRadius,
            textAlign: 'center',
            alignContent: 'center',
            cursor: 'pointer',
            backgroundColor: active ? colors.crimson : colors.mossGray,
          }}
          initial={{
            y: -8,
          }}
          whileHover={{
            y: -5,
          }}
          whileTap={{
            y: 0,
          }}
          onClick={onClick}
          onTapStart={() => {
            new Audio(buttonClick).play()
          }}
        ></motion.button>
      </motion.div>
    </motion.div>
  )
}

const outerStyle = {
  width: cardWidth,
  height: cardHeight,

  position: 'relative',
  display: 'flex',
  alignContent: 'stretch',
  justifyContent: 'stretch',
} satisfies MotionStyle
