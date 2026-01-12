export const choose = <T>(array: readonly T[]) => {
  const index = Math.floor(Math.random() * array.length)
  return array[index]!
}
