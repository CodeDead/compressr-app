import React, { useContext } from "react";
import {
  Center,
  Tooltip,
  UnstyledButton,
  Stack,
  rem,
  Image,
} from "@mantine/core";
import {
  IconHome2,
  IconKey,
  IconSettings,
  IconLogout,
  IconInfoCircle,
} from "@tabler/icons-react";
import classes from "./headerbar.module.css";
import { MainContext } from "../../context/MainContextProvider/index.jsx";
import { useNavigate } from "react-router-dom";

const NavbarLink = ({ icon: Icon, label, active, onClick }) => {
  return (
    <Tooltip label={label} position="right" transitionProps={{ duration: 0 }}>
      <UnstyledButton
        onClick={onClick}
        className={classes.link}
        data-active={active || undefined}
      >
        <Icon style={{ width: rem(20), height: rem(20) }} stroke={1.5} />
      </UnstyledButton>
    </Tooltip>
  );
};

const linkData = [
  { icon: IconHome2, label: "Home", path: "/" },
  { icon: IconInfoCircle, label: "About", path: "/about" },
  { icon: IconSettings, label: "Settings", path: "/settings" },
];

const HeaderBar = () => {
  const [state] = useContext(MainContext);
  const { pageIndex } = state;

  const navigate = useNavigate();

  const links = linkData.map((link, index) => (
    <NavbarLink
      {...link}
      key={link.label}
      active={index === pageIndex}
      onClick={() => navigate(link.path)}
    />
  ));

  return (
    <nav className={classes.navbar}>
      <Center>
        <Image src="/favicon.ico" width={30} height={30} />
      </Center>

      <div className={classes.navbarMain}>
        <Stack justify="center" gap={0}>
          {links}
        </Stack>
      </div>

      <Stack justify="center" gap={0}>
        <NavbarLink icon={IconKey} label="License" />
        <NavbarLink icon={IconLogout} label="Logout" />
      </Stack>
    </nav>
  );
};

export default HeaderBar;
