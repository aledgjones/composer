import { mdiClose } from "@mdi/js";

import { Content } from "../../ui/components/content";
import { Dialog } from "../../ui/components/dialog";
import { Icon } from "../../ui/components/icon";
import { Label } from "../../ui/components/label";
import { Subheader } from "../../ui/components/subheader";

import logo from "./logo-silo.svg";

import "./styles.css";

interface Props {
  onClose: () => void;
}

export const About = Dialog<Props>(({ onClose }) => {
  return (
    <div className="about">
      <div className="about__header">
        <Icon path={mdiClose} size={24} onClick={onClose} />
      </div>
      <div className="about__logo">
        <img className="about__logo-img" alt="Solo Composer Logo" src={logo} />
        <Label className="about__logo-text">
          <p>Solo Composer</p>
          <p>Music notation everywhere</p>
        </Label>
      </div>
      <Content className="about__content">
        <p className="about__warning">
          This project is very much an experimental work in progress. Things{" "}
          <b>will</b> break, not exist, make no sense and crash!
        </p>
      </Content>
      <Content className="about__versions">
        <Subheader>Versions</Subheader>
        <p className="about__version">
          <span className="about__grow">Application UI</span>
          <span>0.50.0</span>
        </p>
        <p className="about__version">
          <span className="about__grow">Application Engine</span>
          <span>0.50.0</span>
        </p>
        <p className="about__version">
          <span className="about__grow">Audio Engine</span>
          <span>1.0.0</span>
        </p>
        <p className="about__version">
          <span className="about__grow">Rendering Engine</span>
          <span>18.0.0</span>
        </p>
      </Content>
    </div>
  );
});
