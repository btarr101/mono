export type DropPointProps = {
  beforeId?: string
  afterId?: string
}

export const DropPoint = ({ beforeId, afterId }: DropPointProps) => (
  <div
    style={{
      width: '0px',
      background: 'red',
      height: '100%',
    }}
    x-after={afterId}
    x-before={beforeId}
    x-drop-point="true"
  ></div>
)
