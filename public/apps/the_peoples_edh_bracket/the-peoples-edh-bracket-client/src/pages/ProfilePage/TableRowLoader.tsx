import { Loader, Table } from '@mantine/core'

export const TableRowLoader = () => (
  <Table.Tr>
    <Table.Td align="center" colSpan={7}>
      <Loader color="blue" my="xl" />
    </Table.Td>
  </Table.Tr>
)
