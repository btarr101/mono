import { type ReactVirtualizer } from '@tanstack/react-virtual'
import { useLayoutEffect, useRef } from 'react'

export const useReactVirtualScrollRestoration = <
  TScrollElement extends Window | Element,
  TItemElement extends Element,
>(
  virtualizer: ReactVirtualizer<TScrollElement, TItemElement>,
) => {
  const savedScroll = useRef<number | null>(null)
  useLayoutEffect(() => {
    if (savedScroll.current !== null) {
      console.log({
        restore: savedScroll.current,
      })
      virtualizer.scrollToOffset(savedScroll.current)
    }
    return () => {
      console.log({
        save: window.scrollY ?? 0,
      })
      savedScroll.current = window.scrollY ?? 0
    }
  }, [virtualizer])
}
