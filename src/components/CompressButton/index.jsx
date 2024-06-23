import React from "react";
import {
  Text,
  Group,
  Button,
  rem,
  Container,
  Paper,
  Center,
  Loader,
} from "@mantine/core";
import { IconWand } from "@tabler/icons-react";

const CompressButton = ({ disabled, loading, onClick }) => {
  return (
    <Paper>
      <Container>
        <div>
          {loading ? (
            <Center>
              <Loader type="bars" />
            </Center>
          ) : (
            <>
              <Group justify="center">
                <IconWand
                  style={{ width: rem(50), height: rem(50) }}
                  stroke={1.5}
                />
              </Group>
              <Text ta="center" fw={700} fz="lg" mt="xl">
                Let&apos;s save some space
              </Text>
            </>
          )}
        </div>
      </Container>
      <Center>
        <Button
          aria-label="Compress"
          size="md"
          mt="md"
          radius="xl"
          disabled={disabled}
          onClick={onClick}
        >
          Compress
        </Button>
      </Center>
    </Paper>
  );
};

export default CompressButton;
