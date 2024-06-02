import React, { useContext, useEffect, useState } from "react";
import DropzoneButton from "../../components/DropzoneButton/index.jsx";
import { Container, Button, Text, Center, Paper } from "@mantine/core";
import { MainContext } from "../../context/MainContextProvider/index.jsx";
import {
  setFiles,
  setPageIndex,
  setQuality,
} from "../../reducer/MainReducer/Actions/index.js";
import CompressSlider from "../../components/CompressSlider/index.jsx";
import { IconCircleLetterX, IconSword } from "@tabler/icons-react";
import { notifications } from "@mantine/notifications";

const Home = () => {
  const [popOverOpen, setpopOverOpen] = useState(false);

  const [state, d1] = useContext(MainContext);
  const { quality, files } = state;

  /**
   * Change the popover open state
   */
  const changePopOverOpen = () => {
    setpopOverOpen((e) => !e);
  };

  /**
   * Change the files
   * @param files The new array of files
   */
  const changeFiles = (files) => {
    d1(setFiles(files));
  };

  /**
   * Change the compression quality level
   * @param quality The new compression quality level
   */
  const changeQuality = (quality) => {
    d1(setQuality(quality));
  };

  /**
   * Compress the files
   */
  const compressFiles = () => {
    if (!files) return;

    let error = null;
    // eslint-disable-next-line no-unused-vars
    for (const c of files) {
      // TODO
    }

    if (!error) {
      notifications.show({
        title: "Success",
        message: "Hey there, your images were compressed successfully! 🤥",
      });
    } else {
      notifications.show({
        title: "Error",
        message: "The image could not be compressed. Please try again. 😢",
      });
    }
  };

  useEffect(() => {
    d1(setPageIndex(0));
    document.title = "Home | Compressr";
  }, []);

  return (
    <Container style={{ height: "100vh" }}>
      <Center style={{ width: "100%", height: "100%" }}>
        {files ? (
          <Paper p="xl" style={{ width: "100%" }}>
            <Text size="md">Quality</Text>
            <CompressSlider value={quality} onChange={changeQuality} />
            <Button
              aria-label="Cancel"
              style={{ float: "left" }}
              mt="xl"
              leftSection={<IconCircleLetterX size={14} />}
              onClick={() => {
                d1(setFiles(null));
              }}
            >
              Cancel
            </Button>
            <Button
              aria-label="Compress"
              style={{ float: "right" }}
              mt="xl"
              leftSection={<IconSword size={14} />}
              onClick={compressFiles}
            >
              Compress
            </Button>
          </Paper>
        ) : (
          <DropzoneButton
            popOverOpen={popOverOpen}
            setPopOverOpen={changePopOverOpen}
            changeFiles={changeFiles}
          />
        )}
      </Center>
    </Container>
  );
};

export default Home;
