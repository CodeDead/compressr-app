import React, { useContext, useEffect } from "react";
import { MainContext } from "../../context/MainContextProvider/index.jsx";
import {
  setAutoUpdate,
  setPageIndex,
  setThemeType,
} from "../../reducer/MainReducer/Actions/index.js";
import {
  Card,
  Container,
  Group,
  Radio,
  Switch,
  useMantineColorScheme,
} from "@mantine/core";

const Settings = () => {
  const [state, dispatch] = useContext(MainContext);
  const { setColorScheme } = useMantineColorScheme();

  const { themeType, autoUpdate } = state;

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
      </Card>
    </Container>
  );
};

export default Settings;
