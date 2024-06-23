import React from "react";
import {
  Text,
  Group,
  rem,
  Container,
  Paper,
  Center,
} from "@mantine/core";
import { IconCloudUpload } from "@tabler/icons-react";
import SplitButton from "../SplitButton/index.jsx";

const DropzoneButton = ({ addFiles, addFolder }) => {
  return (
    <Paper>
      <Container onClick={addFiles}>
        <div style={{ cursor: "pointer" }}>
          <Group justify="center">
            <IconCloudUpload
              style={{ width: rem(50), height: rem(50) }}
              onClick={addFiles}
              stroke={1.5}
            />
          </Group>
          <Text ta="center" fw={700} fz="lg" mt="xl">
            Select image(s) to compress
          </Text>
        </div>
      </Container>
      <Center>
        <SplitButton onSelectFiles={addFiles} onSelectFolder={addFolder} />
      </Center>
    </Paper>
  );
};

export default DropzoneButton;
