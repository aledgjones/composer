import { FC } from "react";
import { mdiCursorDefault, mdiEraser, mdiPen, mdiBoxCutter } from "@mdi/js";
import { useHotkeys } from "react-hotkeys-hook";
import { Snap } from "../../components/snap";
import { useTitle } from "../../ui/hooks/use-title";
import { useRainbow } from "../../ui/hooks/use-rainbow";
import { DragScroll } from "../../ui/components/drag-scroll";
import { Icon } from "../../ui/components/icon";
import { engine, store } from "../../data";
import { Tool } from "../../data/defs";
import { ControlsPlaceholder } from "./controls-placeholder";
import { actions } from "../../data/actions";
import { Controls } from "./controls";
import { Ticks } from "./ticks";
import { PlayHead } from "./play-head";
import { Track } from "./track";
import { TrackPlaceholder } from "./track-placeholder";
import { Zoom } from "../../components/zoom";
import { BottomBar } from "../../components/bottom-bar";

import "./styles.css";

const Play: FC = () => {
  useTitle("Solo Composer | Sequence");

  const flows: string[] = engine.flows;
  const flowKey = store.useState(
    (s) => {
      if (s.flow && flows.includes(s.flow)) {
        return s.flow;
      } else {
        return flows[0];
      }
    },
    [flows]
  );

  const players: string[] = engine.players;
  const tool = store.useState((s) => s.play.tool);
  const zoom = store.useState((s) => s.play.zoom);
  const subdivisions = engine.get_flow_subdivisions(flowKey);

  const colors = useRainbow(players.length);
  const ticks = engine.get_flow_ticks(flowKey);

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
            const instruments = engine.get_player_instruments(playerKey);
            return instruments.map((instrumentKey: string) => {
              if (engine.flow_contains_player(flowKey, playerKey)) {
                return (
                  <Controls
                    key={instrumentKey}
                    flowKey={flowKey}
                    instrumentKey={instrumentKey}
                    color={colors[i]}
                  />
                );
              } else {
                return (
                  <ControlsPlaceholder
                    key={instrumentKey}
                    instrumentKey={instrumentKey}
                    color={colors[i]}
                  />
                );
              }
            });
          })}
        </div>

        <div className="play__right-panel">
          <div className="play__right-panel-content">
            <PlayHead ticks={ticks} zoom={zoom / 100} />
            <div className="play__ticks">
              <Ticks
                isTrack={false}
                ticks={ticks}
                height={48}
                className="play__tick-track"
                zoom={zoom / 100}
              />
            </div>
            {players.map((playerKey, i) => {
              const instruments: string[] =
                engine.get_player_instruments(playerKey);
              return instruments.map((instrumentKey) => {
                if (engine.flow_contains_player(flowKey, playerKey)) {
                  return (
                    <Track
                      key={instrumentKey}
                      instrumentKey={instrumentKey}
                      flowKey={flowKey}
                      color={colors[i]}
                      ticks={ticks}
                      tool={tool}
                      zoom={zoom / 100}
                      subdivisions={subdivisions}
                    />
                  );
                } else {
                  return <TrackPlaceholder key={instrumentKey} />;
                }
              });
            })}
          </div>
        </div>
      </DragScroll>

      <BottomBar>
        <Snap />
        <div />
        <Zoom
          zoom={zoom}
          inc={actions.play.zoom.inc}
          set={actions.play.zoom.set}
          desc={actions.play.zoom.desc}
        />
      </BottomBar>
    </>
  );
};

export default Play;
