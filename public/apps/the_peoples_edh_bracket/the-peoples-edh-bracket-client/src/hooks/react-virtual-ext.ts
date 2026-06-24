import { type ReactVirtualizer } from '@tanstack/react-virtual'
import { useLayoutEffect, useRef } from 'react'

export const useReactVirtualScrollRestoration = <
  TScrollElement extends Window | Element,
  TItemElement extends Element,
>(
  virtualizer: ReactVirtualizer<TScrollElement, TItemElement>,
) => {
  const savedScroll = useRef(0)
  useLayoutEffect(() => {
    virtualizer.scrollToOffset(savedScroll.current)
    return () => {
      savedScroll.current = window.scrollY ?? 0
    }
  }, [virtualizer])
}
