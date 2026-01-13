import { motion, type MotionStyle } from 'motion/react'
import { useContextMenuStore } from '../../contexts/ContextMenuContext'

export type ItemCardContextMenuProps = {
  contextMenuId: string
}

export const ItemCardContextMenu = ({ contextMenuId }: ItemCardContextMenuProps) => {
  const { openState } = useContextMenuStore()
  if (openState?.contextMenuId !== contextMenuId) return

  const style = {
    position: 'fixed',
    top: openState.y,
    left: openState.x,
    zIndex: 9999,
    background: 'blue',
    padding: 4,
  } satisfies MotionStyle

  return <motion.div style={style}>A CONTEXT MENU</motion.div>
}
