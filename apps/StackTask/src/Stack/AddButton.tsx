import { motion } from 'motion/react'
import { cardHeight, cardRadius, cardWidth, colors } from '../style'
import { useEffect, useRef, useState } from 'react'
import { boxShadow } from '../util/css'

export type EndpointProps = {
  onClick?: (content: string) => void
}

export const AddButton = ({ onClick }: EndpointProps) => {
  const [dialogOpen, setDialogOpen] = useState(false)
  const [content, setContent] = useState('')
  const dialogRef = useRef<HTMLDialogElement | null>(null)
  const inputRef = useRef<HTMLInputElement | null>(null)

  const handleDialogOpen = () => setDialogOpen(true)
  const handleDialogClose = () => setDialogOpen(false)
  const handleSubmit = () => {
    if (content.length === 0) return
    onClick?.(content)
    setContent('')
    inputRef.current?.blur()
    handleDialogClose()
  }

  useEffect(() => {
    if (!dialogOpen) return
    dialogRef.current?.showModal()

    const onPointerDown = (event: PointerEvent) => {
      if (!(event.target instanceof Node)) return

      if (!dialogRef.current?.contains(event.target)) {
        handleDialogClose()
      }
    }

    window.addEventListener('pointerdown', onPointerDown)

    requestAnimationFrame(() => {
      inputRef.current?.focus()
    })

    return () => window.removeEventListener('pointerdown', onPointerDown)
  }, [dialogOpen])

  return (
    <>
      <motion.button
        layout
        style={{
          width: cardWidth,
          height: cardHeight,
          borderRadius: cardRadius,
          display: 'flex',
          alignContent: 'stretch',
          justifyContent: 'stretch',
        }}
        whileHover={{ scale: 1.1 }}
        whileTap={{ scale: 0.9 }}
        onClick={handleDialogOpen}
      >
        <div
          style={{
            outline: `4px dashed ${colors.nearBlack}`,
            width: '100%',
            borderRadius: cardRadius,
            margin: 10,
          }}
        ></div>
      </motion.button>
      <motion.dialog
        animate={{
          scale: dialogOpen ? 1 : 0,
        }}
        ref={dialogRef}
        style={{
          border: 'none',
          backgroundColor: 'white',
          borderRadius: cardRadius,
          boxShadow: boxShadow({
            x: '0px',
            y: '16px',
            blur: '24px',
          }),
        }}
        onAnimationComplete={() => {
          if (!dialogOpen) {
            dialogRef.current?.close()
          }
        }}
        onClose={handleDialogClose}
      >
        <motion.form
          onSubmit={event => {
            event.preventDefault()
            handleSubmit()
          }}
        >
          <h1>Enter your task</h1>
          <div
            style={{
              display: 'flex',
              flexDirection: 'row',
              alignItems: 'center',
            }}
          >
            <input
              ref={inputRef}
              style={{
                fontSize: '24px',
                border: 'none',
                width: '100%',
                backgroundColor: colors.ivory,
                marginRight: 8,
              }}
              value={content}
              onChange={event => setContent(event.target.value)}
            />
            <button
              style={{
                cursor: 'pointer',
                borderRadius: cardRadius,
                background: colors.neonGreen,
                aspectRatio: '1/1',
                padding: 2,
                height: '32px',
                width: '32px',
                alignItems: 'center',
                textAlign: 'center',
                fontWeight: 'bold',
              }}
              type="submit"
            >
              +
            </button>
          </div>
        </motion.form>
      </motion.dialog>
    </>
  )
}
