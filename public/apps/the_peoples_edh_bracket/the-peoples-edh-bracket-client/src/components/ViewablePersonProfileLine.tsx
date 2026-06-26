import { Menu } from '@mantine/core'
import { Link } from 'react-router'

import { PersonProfileLine, type PersonProfileLineProps } from './PersonProfileLine'

export type ViewablePersonProfileLineProps = Omit<PersonProfileLineProps, 'children'>

export const ViewablePersonProfileLine = (props: ViewablePersonProfileLineProps) => (
  <PersonProfileLine {...props}>
    <Menu.Item
      component={Link}
      disabled={!props.person?.uuid}
      to={`/community/${props.person?.uuid}`}
    >
      View Profile
    </Menu.Item>
  </PersonProfileLine>
)
