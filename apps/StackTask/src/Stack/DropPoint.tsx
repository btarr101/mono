export type DropPointProps = {
  beforeId?: string
}

export const DropPoint = ({ beforeId: itemId }: DropPointProps) => (
  <div
    style={{
      width: '0px',
      height: '100%',
    }}
    x-drop-point-before={itemId ?? 'NONE'}
  />
)
