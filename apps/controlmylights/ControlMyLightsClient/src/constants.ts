import EASEL_IMAGE from './assets/panel.jpg?vite-image'
export { EASEL_IMAGE }

const getCombinedAspectDesktopString = (imageWidth: number, imageHeight: number) => {
  const numerator = imageHeight + 2 * imageWidth
  const denominator = 2 * imageHeight

  return `${numerator}/${denominator}`
}

/**
 * Aspect ratio of the image used for the easel
 */
export const EASEL_IMAGE_ASPECT = EASEL_IMAGE.width / EASEL_IMAGE.height
export const EASEL_IMAGE_ASPECT_STRING = `${EASEL_IMAGE.width}/${EASEL_IMAGE.height}`

/**
 * Aspect ratio of the palette and easel combined
 */
export const COMBINED_ASPECT_DESKTOP = getCombinedAspectDesktopString(
  EASEL_IMAGE.width,
  EASEL_IMAGE.height,
)
