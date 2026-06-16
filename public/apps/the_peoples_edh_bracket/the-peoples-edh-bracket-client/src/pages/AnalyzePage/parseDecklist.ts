import type { DecklistMaindeckEntry } from '../../types/bindings/DecklistMaindeckEntry'
import { err, ok } from '../../util/result'

const LINE_REGEX = /(\d+)\s+(.*)\s\(.*$/

export const parseDecklist = (decklist: string) => {
  const lines = decklist.split('\n')
  const parsedLines = lines.map(parseLine)

  const [validLines, invalidLines] = parsedLines.reduce<[DecklistMaindeckEntry[], string[]]>(
    ([validLines, invalidLines], result) => {
      if (result.ty === 'ok') {
        validLines.push(result.value)
      } else {
        invalidLines.push(result.error)
      }

      return [validLines, invalidLines]
    },
    [[], []],
  )

  if (invalidLines.length) {
    return err(invalidLines)
  }

  return ok(validLines)
}

const parseLine = (line: string) => {
  const match = line.match(LINE_REGEX)
  const countMatch = match?.[1]
  const nameMatch = match?.[2]

  if (!(countMatch && nameMatch)) {
    return err(line)
  }

  const count = parseInt(countMatch)
  if (count <= 0) {
    return err(line)
  }

  const name = nameMatch.replaceAll(/\s*\/+\s*/g, ' // ')

  return ok({
    count,
    name,
  } satisfies DecklistMaindeckEntry)
}
