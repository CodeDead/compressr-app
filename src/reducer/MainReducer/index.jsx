import {
  SET_AUTO_UPDATE,
  SET_FILES,
  SET_MAX_HEIGHT,
  SET_MAX_SIZE_ENABLED,
  SET_MAX_WIDTH,
  SET_PAGE_INDEX,
  SET_QUALITY,
  SET_THEME_TYPE,
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
    case SET_MAX_SIZE_ENABLED:
      return {
        ...state,
        maxSizeEnabled: action.payload,
      };
    case SET_AUTO_UPDATE:
      localStorage.autoUpdate = action.payload;
      return {
        ...state,
        autoUpdate: action.payload,
      };
    default:
      throw new Error();
  }
};

export default MainReducer;
