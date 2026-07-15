import type { DecklistMaindeckEntry } from '../../types/bindings/DecklistMaindeckEntry'
import { err, ok } from '../../util/result'

const LINE_REGEX = /(?:^(\d+)\s+)?\s*(.*?)(?:\s+\([^)]*\).*)?$/

export const parseDecklist = (decklist: string) => {
  const lines = decklist.split('\n')
  const parsedLines = lines.map(parseLine)

  const [validLines, invalidLines] = parsedLines.reduce<[DecklistMaindeckEntry[], string[]]>(
    ([validLines, invalidLines], result) => {
      if (result.ty === 'ok') {
        if (result.value) validLines.push(result.value)
      } else {
        invalidLines.push(result.error)
      }

      return [validLines, invalidLines]
    },
    [[], []],
  )

  if (invalidLines.length) {
    return err(invalidLines.map(line => `Could not parse '${line}'`))
  }

  const totalCount = validLines.reduce((acc, line) => acc + line.count, 0)
  if (totalCount > 100) {
    return err([`Decklist must have no more than 100 cards, but has ${totalCount}`])
  }

  return ok(validLines)
}

const parseLine = (line: string) => {
  const trimmedLine = line.trim()
  if (!trimmedLine) {
    return ok(null)
  }

  const match = trimmedLine.match(LINE_REGEX)
  const countMatch = match?.[1]
  const nameMatch = match?.[2]

  if (!nameMatch) {
    return err(line)
  }

  const count = countMatch !== undefined ? parseInt(countMatch) : 1
  if (count <= 0) {
    return err(line)
  }

  const name = nameMatch.replaceAll(/\s*\/+\s*/g, ' // ')

  return ok({
    count,
    name,
  } satisfies DecklistMaindeckEntry)
}
