import { useMutation } from '@tanstack/react-query'

import { postAnalyze } from '../api/decks'

export const useAnalyzeDeck = () =>
  useMutation({
    mutationFn: postAnalyze,
  })
