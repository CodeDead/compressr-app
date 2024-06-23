import React, { useEffect, useContext, Suspense, lazy } from "react";
import HeaderBar from "../HeaderBar";
import {
  Center,
  Loader,
  ScrollArea,
  useMantineColorScheme,
} from "@mantine/core";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import { MainContext } from "../../context/MainContextProvider";
import {
  getNumberOfThreads,
  setThreadCount,
} from "../../reducer/MainReducer/Actions/index.js";

const Home = lazy(() => import("../../routes/Home"));
const About = lazy(() => import("../../routes/About"));
const Settings = lazy(() => import("../../routes/Settings"));
const NotFound = lazy(() => import("../../routes/NotFound"));

const App = () => {
  const [state, dispatch] = useContext(MainContext);
  const { setColorScheme } = useMantineColorScheme();

  const { themeType } = state;

  /**
   * Update the number of threads
   */
  const updateThreads = async () => {
    if (!localStorage.threadCount) {
      getNumberOfThreads()
        .then((res) => {
          dispatch(setThreadCount(res));
        })
        .catch(() => {
          dispatch(setThreadCount(1));
        });
    }
  };

  useEffect(() => {
    setColorScheme(themeType);
    updateThreads();
  }, []);

  return (
    <BrowserRouter>
      <div style={{ display: "flex", height: "100vh" }}>
        <div style={{ display: "flex", flexDirection: "row" }}>
          <HeaderBar />
        </div>
        <div style={{ display: "flex", flexDirection: "row", flexGrow: 1 }}>
          <ScrollArea style={{ height: "100vh", width: "100%" }}>
            <Suspense
              fallback={
                <Center h={100}>
                  <Loader type="bars" />
                </Center>
              }
            >
              <Routes>
                <Route exact path="/" element={<Home />} />
                <Route exact path="/about" element={<About />} />
                <Route exact path="/settings" element={<Settings />} />
                <Route path="*" element={<NotFound />} />
              </Routes>
            </Suspense>
          </ScrollArea>
        </div>
      </div>
    </BrowserRouter>
  );
};

export default App;
