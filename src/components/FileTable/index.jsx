import React from "react";
import { Table } from "@mantine/core";
import { ActionIcon } from "@mantine/core";
import { IconCircleX } from "@tabler/icons-react";

const FileTable = ({ elements, onDelete, disabled }) => {
  const rows = elements.map((element) => (
    <Table.Tr key={element}>
      <Table.Td>{element}</Table.Td>
      <Table.Td>
        <ActionIcon
          size="sm"
          variant="default"
          title="Remove"
          disabled={disabled}
          onClick={() => onDelete(element)}
        >
          <IconCircleX />
        </ActionIcon>
      </Table.Td>
    </Table.Tr>
  ));

  return (
    <Table.ScrollContainer>
      <Table stickyHeader>
        <Table.Thead>
          <Table.Tr>
            <Table.Th>File path</Table.Th>
            <Table.Th>#</Table.Th>
          </Table.Tr>
        </Table.Thead>
        <Table.Tbody>{rows}</Table.Tbody>
      </Table>
    </Table.ScrollContainer>
  );
};

export default FileTable;
