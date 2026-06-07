import { Image as MantineImage, type ImageProps, Skeleton, Transition } from '@mantine/core'
import { useState } from 'react'

export const LoadingImage = (props: ImageProps) => {
  const [loaded, setLoaded] = useState(() => {
    const img = new Image()
    img.src = props.src
    return img.complete
  })

  return (
    <>
      <MantineImage onLoad={() => setLoaded(true)} {...props} />
      <Transition duration={200} mounted={!loaded} transition="fade">
        {sytles => (
          // @ts-expect-error just fowarding props...
          <Skeleton {...props} style={sytles} />
        )}
      </Transition>
    </>
  )
}
