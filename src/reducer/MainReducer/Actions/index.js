import {
  SET_AUTO_UPDATE,
  SET_FILES,
  SET_MAX_HEIGHT,
  SET_MAX_SIZE_ENABLED,
  SET_MAX_WIDTH,
  SET_PAGE_INDEX,
  SET_QUALITY,
  SET_THEME_TYPE,
} from "./ActionTypes/index.js";

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

export const setMaxSizeEnabled = (enabled) => ({
  type: SET_MAX_SIZE_ENABLED,
  payload: enabled,
});
