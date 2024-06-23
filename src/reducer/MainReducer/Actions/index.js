import {
  SET_AUTO_UPDATE,
  SET_COMPRESSING,
  SET_DELETE_ORIGINAL_IMAGES,
  SET_FILES,
  SET_MAX_HEIGHT,
  SET_MAX_WIDTH,
  SET_PAGE_INDEX,
  SET_QUALITY,
  SET_THEME_TYPE,
  SET_THREAD_COUNT,
  SET_THREAD_MODE,
} from "./ActionTypes/index.js";
import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";

export const setPageIndex = (index) => ({
  type: SET_PAGE_INDEX,
  payload: index,
});

export const setThemeType = (type) => ({
  type: SET_THEME_TYPE,
  payload: type,
});

export const setFiles = (files) => ({
  type: SET_FILES,
  payload: files,
});

export const setQuality = (quality) => ({
  type: SET_QUALITY,
  payload: quality,
});

export const setAutoUpdate = (autoUpdate) => ({
  type: SET_AUTO_UPDATE,
  payload: autoUpdate,
});

export const setMaxHeight = (maxHeight) => ({
  type: SET_MAX_HEIGHT,
  payload: maxHeight,
});

export const setMaxWidth = (maxWidth) => ({
  type: SET_MAX_WIDTH,
  payload: maxWidth,
});

export const setThreadMode = (mode) => ({
  type: SET_THREAD_MODE,
  payload: mode,
});

export const setThreadCount = (count) => ({
  type: SET_THREAD_COUNT,
  payload: count,
});

export const setCompressing = (compressing) => ({
  type: SET_COMPRESSING,
  payload: compressing,
});

export const setDeleteOriginalImages = (deleteOriginalImages) => ({
  type: SET_DELETE_ORIGINAL_IMAGES,
  payload: deleteOriginalImages,
});

export const getNumberOfThreads = () => invoke("get_number_of_threads");

export const getImagesFromFolder = async () => {
  const selected = await open({
    directory: true,
  });

  return await invoke("get_images_from_directory", { directory: selected });
}
