import React from "react";
import {
  Card,
  Avatar,
  Text,
  Group,
  Button,
  ActionIcon,
  rem,
} from "@mantine/core";
import classes from "./aboutcard.module.css";
import {
  IconBrandGithub,
  IconBrandMastodon,
  IconBrandX,
  IconWorldWww,
} from "@tabler/icons-react";
import pkg from "../../../package.json";
import { invoke } from "@tauri-apps/api";
import { notifications } from "@mantine/notifications";

const AboutCard = () => {
  /**
   * Show an error notification when a link could not be opened
   */
  const showLinkError = () => {
    notifications.show({
      title: "Error",
      message: "The link could not be opened. Please try again. 😢",
    });
  };

  /**
   * Open the CodeDead website in a new tab
   */
  const openCodeDead = () => {
    invoke("open", { site: "https://codedead.com/" }).catch(() => {
      showLinkError();
    });
  };

  /**
   * Open the X.com website in a new tab
   */
  const openX = () => {
    invoke("open", { site: "https://x.com/C0DEDEAD" }).catch(() => {
      showLinkError();
    });
  };

  /**
   * Open the Github profile in a new tab
   */
  const openGithub = () => {
    invoke("open", { site: "https://github.com/CodeDead/" }).catch(() => {
      showLinkError();
    });
  };

  /**
   * Open the Mastodon profile in a new tab
   */
  const openMastodon = () => {
    invoke("open", { site: "https://mstdn.social/@CodeDead" }).catch(() => {
      showLinkError();
    });
  };

  return (
    <Card withBorder radius="md" className={classes.card} mt="xl">
      <Card.Section
        h={140}
        onClick={openCodeDead}
        style={{
          backgroundImage:
            "url(https://images.unsplash.com/photo-1488590528505-98d2b5aba04b?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=500&q=80)",
          cursor: "pointer",
        }}
      />
      <Avatar
        src="https://codedead.com/favicon.ico"
        size={80}
        radius={80}
        mx="auto"
        mt={-30}
        onClick={openCodeDead}
        style={{
          cursor: "pointer",
        }}
        className={classes.avatar}
      />
      <Text
        ta="center"
        fz="lg"
        fw={500}
        mt="sm"
        style={{ cursor: "pointer" }}
        onClick={openCodeDead}
      >
        CodeDead
      </Text>
      <Text mt="sm" ta="center" fz="sm" c="dimmed">
        Compressr, a CodeDead product, was made with ❤️ by DeadLine.
      </Text>
      <Text ta="center" fz="sm" c="dimmed">
        Version: v{pkg.version}
      </Text>
      <Group mt="sm" gap={1} justify="center">
        <ActionIcon
          aria-label="CodeDead"
          size="lg"
          color="gray"
          variant="subtle"
          onClick={openCodeDead}
        >
          <IconWorldWww
            style={{ width: rem(18), height: rem(18) }}
            stroke={1.5}
          />
        </ActionIcon>
        <ActionIcon
          aria-label="X"
          size="lg"
          color="gray"
          variant="subtle"
          onClick={openX}
        >
          <IconBrandX
            style={{ width: rem(18), height: rem(18) }}
            stroke={1.5}
          />
        </ActionIcon>
        <ActionIcon
          aria-label="Mastodon"
          size="lg"
          color="gray"
          variant="subtle"
          onClick={openMastodon}
        >
          <IconBrandMastodon
            style={{ width: rem(18), height: rem(18) }}
            stroke={1.5}
          />
        </ActionIcon>
        <ActionIcon
          aria-label="Github"
          size="lg"
          color="gray"
          variant="subtle"
          onClick={openGithub}
        >
          <IconBrandGithub
            style={{ width: rem(18), height: rem(18) }}
            stroke={1.5}
          />
        </ActionIcon>
      </Group>
      <Button
        aria-label="Update"
        fullWidth
        radius="md"
        mt="md"
        size="md"
        variant="default"
      >
        Check for updates
      </Button>
    </Card>
  );
};

export default AboutCard;
