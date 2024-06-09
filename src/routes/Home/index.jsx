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
} from "@mantine/core";
import { MainContext } from "../../context/MainContextProvider/index.jsx";
import {
  setFiles,
  setPageIndex,
  setQuality,
} from "../../reducer/MainReducer/Actions/index.js";
import CompressSlider from "../../components/CompressSlider/index.jsx";
import { IconCircleX } from "@tabler/icons-react";
import { notifications } from "@mantine/notifications";
import FileTable from "../../components/FileTable/index.jsx";

const Home = () => {
  const [hasSelectedFiles, setHasSelectedFiles] = useState(false);
  const [popOverOpen, setpopOverOpen] = useState(false);
  const [active, setActive] = useState(0);

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
  // eslint-disable-next-line no-unused-vars
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
      <Center style={{ width: "100%", height: "85%" }}>
        {active === 0 ? (
          files ? (
            <Paper p="xl" style={{ width: "100%" }}>
              <ScrollArea h={250}>
                <FileTable elements={files} onDelete={removeFile} />
              </ScrollArea>
              <Button size="md" mt="md" onClick={() => changeFiles(null)}>
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
            <CompressSlider value={quality} onChange={changeQuality} />
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
