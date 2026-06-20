import { Menu } from '@mantine/core'

import { PersonProfileLine, type PersonProfileLineProps } from './PersonProfileLine'

export type ViewablePersonProfileLineProps = Omit<PersonProfileLineProps, 'children'>

export const ViewablePersonProfileLine = (props: ViewablePersonProfileLineProps) => (
  <PersonProfileLine {...props}>
    <Menu.Item>View Profile</Menu.Item>
  </PersonProfileLine>
)
