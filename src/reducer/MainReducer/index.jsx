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
} from "./Actions/ActionTypes/index.js";

const MainReducer = (state, action) => {
  switch (action.type) {
    case SET_PAGE_INDEX:
      return {
        ...state,
        pageIndex: action.payload,
      };
    case SET_THEME_TYPE:
      localStorage.themeType = action.payload;
      return {
        ...state,
        themeType: action.payload,
      };
    case SET_FILES:
      return {
        ...state,
        files: action.payload,
      };
    case SET_QUALITY:
      return {
        ...state,
        quality: action.payload,
      };
    case SET_MAX_WIDTH:
      return {
        ...state,
        maxWidth: action.payload,
      };
    case SET_MAX_HEIGHT:
      return {
        ...state,
        maxHeight: action.payload,
      };
    case SET_AUTO_UPDATE:
      localStorage.autoUpdate = action.payload;
      return {
        ...state,
        autoUpdate: action.payload,
      };
    case SET_THREAD_MODE:
      localStorage.threadMode = action.payload;
      return {
        ...state,
        threadMode: action.payload,
      };
    case SET_THREAD_COUNT:
      localStorage.threadCount = action.payload;
      return {
        ...state,
        threadCount: action.payload,
      };
    case SET_COMPRESSING:
      return {
        ...state,
        compressing: action.payload,
      };
    case SET_DELETE_ORIGINAL_IMAGES:
      return {
        ...state,
        deleteOriginalImages: action.payload,
      };
    default:
      throw new Error();
  }
};

export default MainReducer;
