import React, { useContext, useEffect, useState } from "react";
import DropzoneButton from "../../components/DropzoneButton/index.jsx";
import {
  Container,
  Text,
  Center,
  Paper,
  Stepper,
  Button,
  ScrollArea,
  NumberInput,
  Loader,
} from "@mantine/core";
import { MainContext } from "../../context/MainContextProvider/index.jsx";
import {
  getNumberOfThreads,
  setCompressing,
  setFiles,
  setMaxHeight,
  setMaxWidth,
  setPageIndex,
  setQuality,
} from "../../reducer/MainReducer/Actions/index.js";
import CompressSlider from "../../components/CompressSlider/index.jsx";
import { IconCircleX } from "@tabler/icons-react";
import { notifications } from "@mantine/notifications";
import FileTable from "../../components/FileTable/index.jsx";
import { invoke } from "@tauri-apps/api";

const Home = () => {
  const [hasSelectedFiles, setHasSelectedFiles] = useState(false);
  const [popOverOpen, setpopOverOpen] = useState(false);
  const [active, setActive] = useState(0);

  const [state, d1] = useContext(MainContext);
  const {
    compressing,
    quality,
    files,
    maxWidth,
    maxHeight,
    threadMode,
    threadCount,
  } = state;

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

    if (!hasSelectedFiles && files) {
      setHasSelectedFiles(true);
      setActive(1);
    }
  };

  /**
   * Change the compression quality level
   * @param quality The new compression quality level
   */
  const changeQuality = (quality) => {
    d1(setQuality(quality));
  };

  const changeMaxWidth = (maxWidth) => {
    d1(setMaxWidth(maxWidth));
  };

  const changeMaxHeight = (maxHeight) => {
    d1(setMaxHeight(maxHeight));
  };

  /**
   * Remove a file from the files array
   * @param file The file to remove
   */
  const removeFile = (file) => {
    let newFiles = files.filter((f) => f !== file);
    if (newFiles.length === 0) {
      newFiles = null;
    }
    d1(setFiles(newFiles));
  };

  /**
   * Compress the files
   */
  const compressFiles = async () => {
    if (!files) return;

    d1(setCompressing(true));

    let threads = 1;
    if (threadMode === "manual") {
      threads = threadCount;
    } else if (threadMode === "auto") {
      threads = await getNumberOfThreads();
    }

    invoke("compress_image", {
      files,
      quality: parseFloat(quality),
      maxWidth: parseFloat(maxWidth ? maxWidth : 0),
      maxHeight: parseFloat(maxHeight ? maxHeight : 0),
      numThreads: threads,
    })
      .catch((e) => {
        notifications.show({
          title: "Error",
          message:
            "The image could not be compressed. Please try again 😢: " + e,
        });
      })
      .finally(() => {
        d1(setCompressing(false));
      });
  };

  useEffect(() => {
    d1(setPageIndex(0));
    document.title = "Home | Compressr";
  }, []);

  return (
    <Container style={{ height: "100vh" }}>
      <Center style={{ width: "100%", height: "85%" }}>
        {active === 0 ? (
          files ? (
            <Paper p="xl" style={{ width: "100%" }}>
              <ScrollArea h={250}>
                <FileTable
                  elements={files}
                  onDelete={removeFile}
                  disabled={compressing}
                />
              </ScrollArea>
              <Button
                size="md"
                mt="md"
                onClick={() => changeFiles(null)}
                disabled={compressing}
              >
                Clear
              </Button>
            </Paper>
          ) : (
            <DropzoneButton
              popOverOpen={popOverOpen}
              setPopOverOpen={changePopOverOpen}
              changeFiles={changeFiles}
            />
          )
        ) : null}
        {active === 1 ? (
          <Paper p="xl" style={{ width: "100%" }}>
            <Text size="md">Quality</Text>
            <CompressSlider
              value={quality}
              onChange={changeQuality}
              disabled={compressing}
            />
            <NumberInput
              mt="xl"
              label="Maximum width"
              disabled={compressing}
              min={1}
              value={maxWidth}
              onChange={changeMaxWidth}
              description="Resize an image if it is larger than the specified width in pixels"
              placeholder="Leave empty to disable"
            />
            <NumberInput
              mt="sm"
              label="Maximum height"
              disabled={compressing}
              min={1}
              value={maxHeight}
              onChange={changeMaxHeight}
              description="Resize an image if it is larger than the specified height in pixels"
              placeholder="Leave empty to disable"
            />
          </Paper>
        ) : null}
        {active === 2 ? (
          <Paper p="lg" style={{ width: "100%" }}>
            {compressing ? (
              <Center>
                <Loader type="bars" />
              </Center>
            ) : null}
            <Center>
              <Button
                size="md"
                mt="xl"
                onClick={compressFiles}
                disabled={!files || compressing}
              >
                Compress
              </Button>
            </Center>
          </Paper>
        ) : null}
      </Center>
      <Stepper active={active} onStepClick={setActive}>
        <Stepper.Step
          label="Step 1"
          description="Image(s)"
          completedIcon={files ? null : <IconCircleX />}
          color={files ? null : active === 0 ? null : "red"}
        />
        <Stepper.Step label="Step 2" description="Options" />
        <Stepper.Step label="Step 3" description="Compress" />
        <Stepper.Completed>
          Completed, click back button to get to previous step
        </Stepper.Completed>
      </Stepper>
    </Container>
  );
};

export default Home;
