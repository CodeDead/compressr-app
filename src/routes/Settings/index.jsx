import React, { useContext, useEffect } from "react";
import { MainContext } from "../../context/MainContextProvider/index.jsx";
import {
  setAutoUpdate,
  setPageIndex,
  setThemeType,
  setThreadCount,
  setThreadMode,
} from "../../reducer/MainReducer/Actions/index.js";
import {
  Card,
  Container,
  Group,
  NumberInput,
  Radio,
  Switch,
  useMantineColorScheme,
} from "@mantine/core";

const Settings = () => {
  const [state, dispatch] = useContext(MainContext);
  const { setColorScheme } = useMantineColorScheme();

  const { themeType, autoUpdate, threadMode, threadCount } = state;

  /**
   * Change the theme type
   */
  const changeTheme = (value) => {
    dispatch(setThemeType(value));
    setColorScheme(value);
  };

  useEffect(() => {
    dispatch(setPageIndex(2));
  }, []);

  return (
    <Container size="sm">
      <Card mt="xl" shadow="sm" radius="md" withBorder>
        <Switch
          label="Automatically check for updates"
          description="Let the application notify you when there are updates available."
          checked={autoUpdate}
          onChange={(event) => {
            dispatch(setAutoUpdate(event.currentTarget.checked));
          }}
        />
        <Switch
          mt="xl"
          label="Automatic multithreading"
          description="Let the application decide how many threads to use."
          checked={threadMode === "auto"}
          onChange={(event) => {
            if (event.currentTarget.checked) {
              dispatch(setThreadMode("auto"));
            } else {
              dispatch(setThreadMode("manual"));
            }
          }}
        />
        <Radio.Group
          mt="sm"
          name="themeType"
          label="Theme"
          description="How do you want the application to look?"
          value={themeType}
          onChange={changeTheme}
        >
          <Group mt="xs">
            <Radio label="Auto" value="auto" />
            <Radio label="Dark" value="dark" />
            <Radio label="Light" value="light" />
          </Group>
        </Radio.Group>
        {threadMode !== "auto" ? (
          <NumberInput
            label="Thread count"
            description="How many threads do you want to use?"
            mt="md"
            disabled={threadMode === "auto"}
            min={1}
            value={threadCount}
            onChange={(value) => {
              dispatch(setThreadCount(value));
            }}
          />
        ) : null}
      </Card>
    </Container>
  );
};

export default Settings;
