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
          <Accordion.Item className={classes.item} value="output-look">
            <Accordion.Control>
              What will the output look like?
            </Accordion.Control>
            <Accordion.Panel>
              The output will look very similar to the original image, but with
              a smaller file size, depending on the compression level you
              choose.
            </Accordion.Panel>
          </Accordion.Item>

          <Accordion.Item className={classes.item} value="output-format">
            <Accordion.Control>
              What format does the output use?
            </Accordion.Control>
            <Accordion.Panel>
              The compressed image will be a JPEG image, regardless of the input
              format. This is because JPEG is the most widely supported image
              format and because it provides a high level of compression while
              maintaining a good level of quality.
            </Accordion.Panel>
          </Accordion.Item>

          <Accordion.Item className={classes.item} value="server-upload">
            <Accordion.Control>
              Do my images get uploaded to a remote server?
            </Accordion.Control>
            <Accordion.Panel>
              No. Compression is done entirely on your own device and your
              images do not get uploaded anywhere.
            </Accordion.Panel>
          </Accordion.Item>

          <Accordion.Item className={classes.item} value="commercial-purpose">
            <Accordion.Control>
              Can I use this tool for commercial purposes?
            </Accordion.Control>
            <Accordion.Panel>
              Yes! You can use this tool for any purpose, including commercial
              ones.
            </Accordion.Panel>
          </Accordion.Item>

          <Accordion.Item className={classes.item} value="limitations">
            <Accordion.Control>
              Can I use this tool without any limitations?
            </Accordion.Control>
            <Accordion.Panel>
              Yes! You can use this tool without any limitations.
            </Accordion.Panel>
          </Accordion.Item>

          <Accordion.Item className={classes.item} value="maximum-size">
            <Accordion.Control>
              What is the maximum file size I can compress?
            </Accordion.Control>
            <Accordion.Panel>
              The maximum file size depends on your hardware. If you want to
              compress a very large image, or if you are processing a lot of
              images, the compression process might take a while.
            </Accordion.Panel>
          </Accordion.Item>

          <Accordion.Item className={classes.item} value="pixel-art">
            <Accordion.Control>
              Can I create pixel art with this tool?
            </Accordion.Control>
            <Accordion.Panel>
              Yes! If you set the compression level between 0 and 10, your
              images will be very pixelated.
            </Accordion.Panel>
          </Accordion.Item>
        </Accordion>
      </Container>
    </Container>
  );
};

export default About;
