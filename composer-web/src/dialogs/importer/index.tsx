import { useState, useEffect } from "react";
import { actions } from "../../data/actions";
import { Content } from "../../ui/components/content";
import { Dialog } from "../../ui/components/dialog";
import { Progress } from "../../ui/components/progress";
import { Subheader } from "../../ui/components/subheader";

import "./styles.css";

interface Props {
  onClose: () => void;
}

export const Importer = Dialog<Props>(({ onClose }) => {
  const [total, setTotal] = useState(1);
  const [progress, setProgress] = useState(0);

  useEffect(() => {
    const run = async () => {
      try {
        await actions.app.open((progress, total) => {
          setTotal(total);
          setProgress(progress);
        });
      } catch (e) {
        console.log(e);
      }
      onClose();
    };
    run();
  }, []);

  return (
    <div className="importer">
      <Content>
        <Subheader>Loading...</Subheader>
        <Progress color="#cccccc" percent={(progress / total) * 100} />
        <p className="importer__text">
          {((progress / total) * 100).toFixed(0)}%
        </p>
      </Content>
    </div>
  );
});
