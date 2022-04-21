import { FC } from "react";
import {
  mdiCursorDefault,
  mdiEraser,
  mdiPen,
  mdiBoxCutter,
  mdiPlus,
  mdiMinus,
} from "@mdi/js";
import { useHotkeys } from "react-hotkeys-hook";
import { Snap } from "../../components/snap";
import { useTitle } from "../../ui/hooks/use-title";
import { useRainbow } from "../../ui/hooks/use-rainbow";
import { DragScroll } from "../../ui/components/drag-scroll";
import { Icon } from "../../ui/components/icon";
import { Select } from "../../ui/components/select";
import { Option } from "../../ui/components/option";
import { engine, ui } from "../../data";
import { Tool } from "../../data/defs";
import { Controls } from "./controls";
import { actions } from "../../data/actions";

import "./styles.css";

const Play: FC = () => {
  useTitle("Solo Composer | Sequence");

  const players: string[] = engine.players;
  const tool = ui.useState((s) => s.play.tool);
  const zoom = ui.useState((s) => s.play.zoom);

  const colors = useRainbow(players.length);

  // const [flowKey, players, tool, zoom] = useStore((s) => {
  //   const flowKey = s.ui.flowKey;
  //   const flowPlayers = s.score.flows.byKey[flowKey].players;
  //   return [
  //     flowKey,
  //     s.score.players.order
  //       .filter((playerKey) => flowPlayers[playerKey])
  //       .map((playerKey) => s.score.players.byKey[playerKey]),
  //     s.ui.play.tool,
  //     s.ui.play.zoom,
  //   ];
  // });

  // const ticks = useTicks();

  useHotkeys("1", () => actions.play.tool.set(Tool.Select));
  useHotkeys("2", () => actions.play.tool.set(Tool.Draw));
  useHotkeys("3", () => actions.play.tool.set(Tool.Slice));
  useHotkeys("4", () => actions.play.tool.set(Tool.Erase));

  return (
    <>
      <DragScroll className="play__content" x ignore="no-scroll">
        <div className="play__left-panel no-scroll">
          <div className="play__tools">
            <Icon
              className="play__tool"
              toggled={tool === Tool.Select}
              onClick={() => actions.play.tool.set(Tool.Select)}
              path={mdiCursorDefault}
              size={24}
            />
            <Icon
              className="play__tool"
              toggled={tool === Tool.Draw}
              onClick={() => actions.play.tool.set(Tool.Draw)}
              path={mdiPen}
              size={24}
            />
            <Icon
              className="play__tool"
              toggled={tool === Tool.Slice}
              onClick={() => actions.play.tool.set(Tool.Slice)}
              path={mdiBoxCutter}
              size={24}
            />
            <Icon
              className="play__tool"
              toggled={tool === Tool.Erase}
              onClick={() => actions.play.tool.set(Tool.Erase)}
              path={mdiEraser}
              size={24}
            />
          </div>
          {players.map((playerKey, i) => {
            const instruments: string[] =
              engine.get_player_instruments(playerKey);
            return instruments.map((instrumentKey) => {
              return (
                <Controls
                  key={instrumentKey}
                  instrumentKey={instrumentKey}
                  color={colors[i]}
                />
              );
            });
          })}
        </div>

        <div className="play__right-panel">
          <div className="play__right-panel-content">
            {/* <PlayHead ticks={ticks} zoom={zoom} /> */}
            {/* <div className="play__ticks">
              <Ticks
                isTrack={false}
                ticks={ticks}
                height={48}
                className="play__tick-track"
                zoom={zoom}
              />
            </div> */}
            {/* {players.map((player, i) => {
              return player.instruments.map((instrumentKey) => {
                return (
                  <Track
                    key={instrumentKey}
                    instrumentKey={instrumentKey}
                    flowKey={flowKey}
                    color={colors[i]}
                    ticks={ticks}
                    zoom={zoom}
                  />
                );
              });
            })} */}
          </div>
        </div>
      </DragScroll>

      <div className="play__bottom-panel">
        <Snap />
        <div />
        <div className="play__bottom-panel-section">
          <Icon
            className="play__bottom-panel-icon"
            path={mdiMinus}
            size={22}
            onClick={actions.play.zoom.desc}
          />
          <Select
            className="play__bottom-panel-select play__zoom-select"
            direction="up"
            value={zoom}
            onChange={actions.play.zoom.set}
          >
            {/* This is a bit weired but we need a fake option to hold the current,
                possibly abartrary, zoom level. It is hidden with CSS */}
            <Option value={zoom} displayAs={`${zoom}%`} />
            <Option value={25} displayAs="25%">
              25%
            </Option>
            <Option value={50} displayAs="50%">
              50%
            </Option>
            <Option value={75} displayAs="75%">
              75%
            </Option>
            <Option value={100} displayAs="100%">
              100%
            </Option>
            <Option value={150} displayAs="150%">
              150%
            </Option>
            <Option value={200} displayAs="200%">
              200%
            </Option>
            <Option value={300} displayAs="300%">
              300%
            </Option>
            <Option value={400} displayAs="400%">
              400%
            </Option>
            <Option value={500} displayAs="500%">
              500%
            </Option>
          </Select>
          <Icon
            className="play__bottom-panel-icon"
            path={mdiPlus}
            size={22}
            onClick={actions.play.zoom.inc}
          />
        </div>
      </div>
    </>
  );
};

export default Play;
