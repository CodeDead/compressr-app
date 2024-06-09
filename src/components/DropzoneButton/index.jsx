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
import { open } from "@tauri-apps/api/dialog";

const DropzoneButton = ({ changeFiles }) => {
  const openDialog = async () => {
    const selected = await open({
      multiple: true,
      filters: [
        {
          name: "Image",
          directory: true,
          extensions: [
            "avif",
            "bmp",
            "dds",
            "farbfeld",
            "gif",
            "hdr",
            "ico",
            "jpeg",
            "jpg",
            "exr",
            "png",
            "pnm",
            "qoi",
            "tga",
            "tiff",
            "webp",
          ],
        },
      ],
    });

    changeFiles(selected);
  };

  return (
    <Paper>
      <Container onClick={openDialog}>
        <div style={{ cursor: "pointer" }}>
          <Group justify="center">
            <IconCloudUpload
              style={{ width: rem(50), height: rem(50) }}
              onClick={openDialog}
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
          onClick={openDialog}
        >
          Get started
        </Button>
      </Center>
    </Paper>
  );
};

export default DropzoneButton;
