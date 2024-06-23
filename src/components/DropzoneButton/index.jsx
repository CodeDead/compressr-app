import React from "react";
import {
  Text,
  Group,
  Button,
  rem,
  Container,
  Paper,
  Center,
} from "@mantine/core";
import { IconCloudUpload } from "@tabler/icons-react";

const DropzoneButton = ({ addFiles }) => {
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
        <Button
          aria-label="Compress"
          size="md"
          mt="md"
          radius="xl"
          onClick={addFiles}
        >
          Get started
        </Button>
      </Center>
    </Paper>
  );
};

export default DropzoneButton;
