import { FC, useEffect, useRef, useState } from "react";
import { mdiMenu, mdiOpenInNew, mdiPencilOutline } from "@mdi/js";
import merge from "classnames";

import { engine } from "../../engine";

import { About } from "../../dialogs/about";
import { Card } from "../../ui/components/card";
import { Content } from "../../ui/components/content";
import { Divider } from "../../ui/components/divider";
import { Duration } from "../duration";
import { Icon } from "../../ui/components/icon";
import { Label } from "../../ui/components/label";
import { Link } from "../../ui/components/link";
import { List } from "../../ui/components/list";
import { ListItem } from "../../ui/components/list-item";
import { Meta } from "../../dialogs/meta/meta";
import { Subheader } from "../../ui/components/subheader";
import { Text } from "../text";

import "./styles.css";

export const File: FC = () => {
  const title = engine.title;
  const created = engine.created;

  const update = false;

  const element = useRef<HTMLDivElement>(null);
  const [open, setOpen] = useState(false);
  const [meta, setMeta] = useState(false);
  const [about, setAbout] = useState(process.env.NODE_ENV === "production");

  // auto close
  useEffect(() => {
    const cb = (e: any) => {
      if (!element.current || !element.current.contains(e.target)) {
        setOpen(false);
      }
    };
    document.addEventListener("click", cb);
    return () => document.removeEventListener("click", cb);
  }, [element]);

  return (
    <>
      <div className="file-menu__container" ref={element}>
        {!open && update && (
          <div className="file-menu__dot file-menu__dot--badge" />
        )}
        <Icon
          className="file-menu__icon"
          path={mdiMenu}
          size={24}
          onClick={() => setOpen((o) => !o)}
        />
        {open && (
          <Card className="file-menu">
            <Content>
              <Subheader compact>Current Project</Subheader>
            </Content>
            <ListItem className="file-menu__meta">
              <Label style={{ paddingRight: 16 }}>
                <p className={merge({ "file-menu__undefined": !title })}>
                  <Text content={title || "Untitled Project"} />
                </p>
                <p>
                  Created <Duration when={created} />
                </p>
              </Label>
              <Icon
                path={mdiPencilOutline}
                size={24}
                onClick={() => {
                  setMeta(true);
                  setOpen(false);
                }}
              />
            </ListItem>
            {/* <Divider compact />
            <List onClick={() => setOpen(false)}>
              <ListItem onClick={() => setImporter(true)}>Open...</ListItem>
            </List>
            <Divider compact />
            <List onClick={() => setOpen(false)}>
              <ListItem disabled={!file} onClick={noop}>
                Save
              </ListItem>
              <ListItem onClick={noop}>Save As</ListItem>
            </List> */}
            <Divider compact />
            <List onClick={() => setOpen(false)}>
              {/* <ListItem onClick={() => setPreferences(true)}>
                Preferences
              </ListItem>
              <Divider /> */}
              {/* {update && (
                <>
                  <ListItem onClick={actions.app.update.invoke}>
                    <Label>
                      <p>Update available</p>
                      <p>Restart to apply update now...</p>
                    </Label>
                    <div className="file-menu__dot" />
                  </ListItem>
                  <Divider />
                </>
              )} */}
              <Link
                href="https://aledjones-viola.gitbook.io/solo-composer/"
                target="_blank"
              >
                <ListItem className="ui-list-item--hover">
                  <Label>
                    <p>Help &amp; Feedback</p>
                  </Label>
                  <Icon path={mdiOpenInNew} size={20} />
                </ListItem>
              </Link>
              <ListItem onClick={() => setAbout(true)}>About</ListItem>
            </List>
          </Card>
        )}
      </div>

      <Meta width={900} open={meta} onClose={() => setMeta(false)} />
      <About width={400} open={about} onClose={() => setAbout(false)} />
      {/* <Preferences
        open={preferences}
        width={900}
        onClose={() => setPreferences(false)}
      />
      <Importer
        width={300}
        open={importer}
        onClose={() => setImporter(false)}
      /> */}
    </>
  );
};
