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
  Switch,
} from "@mantine/core";
import { MainContext } from "../../context/MainContextProvider/index.jsx";
import {
  getNumberOfThreads,
  setCompressing,
  setDeleteOriginalImages,
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
import CompressButton from "../../components/CompressButton/index.jsx";
import { open } from "@tauri-apps/api/dialog";

const Home = () => {
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
    deleteOriginalImages,
  } = state;

  /**
   * Change the popover open state
   */
  const changePopOverOpen = () => {
    setpopOverOpen((e) => !e);
  };

  /**
   * Add files to the files array
   * @returns {Promise<void>} The selected files
   */
  const addFiles = async () => {
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

    if (!selected || selected.length === 0) return;

    let filesToAdd = [];
    for (const file of selected) {
      if (!files || !files.includes(file)) {
        filesToAdd.push(file);
      }
    }

    let newFiles = files ? files.concat(filesToAdd) : filesToAdd;
    d1(setFiles(newFiles));
  };

  /**
   * Change the compression quality level
   * @param quality The new compression quality level
   */
  const changeQuality = (quality) => {
    d1(setQuality(quality));
  };

  /**
   * Change the maximum image width
   * @param maxWidth The new maximum image width
   */
  const changeMaxWidth = (maxWidth) => {
    d1(setMaxWidth(maxWidth));
  };

  /**
   * Change the maximum image height
   * @param maxHeight The new maximum image height
   */
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
      deleteOriginal: deleteOriginalImages,
    })
      .then((res) => {
        if (res.length === 0) {
          notifications.show({
            title: "Success",
            message: "Your image(s) have been compressed successfully 🎉",
          });
        } else if (files.length === 1) {
          notifications.show({
            title: "Error",
            message:
              "The image could not be compressed. Please try again 😢: " + res,
          });
        } else if (files.length > 1) {
          notifications.show({
            title: "Success",
            message:
              "Your image(s) have been compressed successfully, however some images could not be compressed. Please try again 😢: " +
              res,
          });
        }

        if (deleteOriginalImages) {
          d1(setFiles(null));
          setActive(0);
        }
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
                radius="xl"
                onClick={() => d1(setFiles(null))}
                disabled={compressing}
                style={{ float: "left" }}
              >
                Clear
              </Button>
              <Button
                size="md"
                mt="md"
                radius="xl"
                onClick={addFiles}
                disabled={compressing}
                style={{ float: "right" }}
              >
                Add
              </Button>
            </Paper>
          ) : (
            <DropzoneButton
              popOverOpen={popOverOpen}
              setPopOverOpen={changePopOverOpen}
              addFiles={addFiles}
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
            <Switch
              mt="md"
              label="Delete original images after compression"
              color="red"
              checked={deleteOriginalImages}
              onChange={(event) =>
                d1(setDeleteOriginalImages(event.currentTarget.checked))
              }
            />
          </Paper>
        ) : null}
        {active === 2 ? (
          <Paper p="lg" style={{ width: "100%" }}>
            <CompressButton
              loading={compressing}
              disabled={!files || compressing}
              onClick={compressFiles}
            />
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
