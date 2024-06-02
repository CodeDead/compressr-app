import React, { useContext, useEffect } from "react";
import classes from "./about.module.css";
import { Accordion, Container, Title } from "@mantine/core";
import { MainContext } from "../../context/MainContextProvider/index.jsx";
import { setPageIndex } from "../../reducer/MainReducer/Actions/index.js";
import AboutCard from "../../components/AboutCard/index.jsx";

const About = () => {
  const [, d1] = useContext(MainContext);

  useEffect(() => {
    d1(setPageIndex(1));
    document.title = "About | Compressr";
  }, []);

  return (
    <Container size="sm">
      <AboutCard />
      <Container className={classes.wrapper}>
        <Title ta="center" className={classes.title}>
          Frequently Asked Questions
        </Title>

        <Accordion variant="separated">
          <Accordion.Item className={classes.item} value="reset-password">
            <Accordion.Control>
              Can I create pixel art with this tool?
            </Accordion.Control>
            <Accordion.Panel>
              Yes! If you set the compression level to 0, your images will be
              very pixelated.
            </Accordion.Panel>
          </Accordion.Item>

          <Accordion.Item className={classes.item} value="another-account">
            <Accordion.Control>Do you store my images?</Accordion.Control>
            <Accordion.Panel>
              No. Compression is entirely done on your own device and your
              images do not pass our servers.
            </Accordion.Panel>
          </Accordion.Item>

          <Accordion.Item className={classes.item} value="newsletter">
            <Accordion.Control>
              Can I use this tool for commercial purposes?
            </Accordion.Control>
            <Accordion.Panel>
              Yes! You can use this tool for any purpose, including commercial
              ones.
            </Accordion.Panel>
          </Accordion.Item>

          <Accordion.Item className={classes.item} value="credit-card">
            <Accordion.Control>
              Can I use this tool without any limitations?
            </Accordion.Control>
            <Accordion.Panel>
              Yes! You can use this tool without any limitations.
            </Accordion.Panel>
          </Accordion.Item>

          <Accordion.Item className={classes.item} value="payment">
            <Accordion.Control>
              What is the maximum file size I can compress?
            </Accordion.Control>
            <Accordion.Panel>
              The maximum file size depends on your hardware. If you want to
              compress a very large image, or if you are processing a lot of
              images, the compression process might take a while.
            </Accordion.Panel>
          </Accordion.Item>
        </Accordion>
      </Container>
    </Container>
  );
};

export default About;
