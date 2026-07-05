import { Loader, Table } from '@mantine/core'

export type TableRowLoaderProps = {
  colSpan: number
}

export const TableRowLoader = ({ colSpan }: TableRowLoaderProps) => (
  <Table.Tr>
    <Table.Td align="center" colSpan={colSpan}>
      <Loader color="blue" my="xl" />
    </Table.Td>
  </Table.Tr>
)
