import React from "react";
import {
  Button,
  Menu,
  Group,
  ActionIcon,
  rem,
  useMantineTheme,
} from "@mantine/core";
import { IconChevronDown, IconFolder } from "@tabler/icons-react";
import classes from "./splitbutton.module.css";

const SplitButton = ({ onSelectFiles, onSelectFolder }) => {
  const theme = useMantineTheme();

  return (
    <Group wrap="nowrap" gap={0}>
      <Button
        mt="md"
        className={classes.button}
        onClick={onSelectFiles}
      >
        Get started
      </Button>
      <Menu
        transitionProps={{ transition: "pop" }}
        position="bottom-end"
        mt="md"
        withinPortal
      >
        <Menu.Target>
          <ActionIcon
            variant="filled"
            color={theme.primaryColor}
            size={36}
            className={classes.menuControl}
          >
            <IconChevronDown
              style={{ width: rem(16), height: rem(16) }}
              stroke={1.5}
            />
          </ActionIcon>
        </Menu.Target>
        <Menu.Dropdown>
          <Menu.Item
            onClick={onSelectFolder}
            leftSection={
              <IconFolder
                style={{ width: rem(16), height: rem(16) }}
                stroke={1.5}
                color={theme.colors.blue[5]}
              />
            }
          >
            Select a folder
          </Menu.Item>
        </Menu.Dropdown>
      </Menu>
    </Group>
  );
};

export default SplitButton;
