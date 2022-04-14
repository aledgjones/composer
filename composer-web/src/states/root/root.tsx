import { FC, Suspense, lazy } from "react";
import { mdiRedo, mdiUndo } from "@mdi/js";

import { ui } from "../../data";
import { View } from "../../data/defs";

import { File } from "../../components/file";
import { Icon } from "../../ui/components/icon";
import { Loading } from "../../components/loading";
import { Tab } from "../../ui/components/tab";
import { Tabs } from "../../ui/components/tabs";

import "./root.css";

const Setup = lazy(() => import("../setup/setup"));

export const Root: FC = () => {
  const view = ui.useState((s) => s.view);

  return (
    <div className="root">
      <div className="root__title-bar">
        <File />
        <Tabs
          className="root__tabs"
          value={view}
          onChange={(value: View) => {
            ui.update((s) => {
              s.view = value;
            });
          }}
        >
          <Tab value={View.Setup}>Setup</Tab>
          <Tab value={View.Write}>Write</Tab>
          <Tab value={View.Engrave}>Engrave</Tab>
          <Tab value={View.Play}>Sequence</Tab>
          <Tab value={View.Print}>Publish</Tab>
        </Tabs>
        {/* <TransportComponent /> */}
        <div className="root__spacer" />
        <div className="root__history">
          <Icon
            disabled
            onClick={() => false}
            className="root__history-icon"
            size={24}
            path={mdiUndo}
          />
          <Icon disabled onClick={() => false} size={24} path={mdiRedo} />
        </div>
      </div>

      <div className="root__content">
        <Suspense fallback={<Loading />}>
          {view === View.Setup && <Setup />}
          {/* {view === View.Write && <Write />} */}
          {/* {tab === TabState.engrave && <Engrave />} */}
          {/* {view === View.Play && <Play />} */}
          {/* {tab === TabState.print && <Fallback color={theme.background[500].fg} type="empty" />} */}
        </Suspense>
      </div>
    </div>
  );
};
