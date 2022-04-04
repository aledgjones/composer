import { engine } from "../../engine";

import { Button } from "../../ui/components/button";
import { Dialog } from "../../ui/components/dialog";
import { Input } from "../../ui/components/input";
import { Subheader } from "../../ui/components/subheader";
import { TagCopier } from "../../components/tag-copier";
import { Textarea } from "../../ui/components/textarea";

import "../generic-settings.css";

interface Props {
  onClose: () => void;
}

export const Meta = Dialog<Props>(({ onClose }) => {
  return (
    <div className="meta">
      <div className="generic-settings__content">
        <div className="generic-settings__section">
          <Subheader style={{ paddingLeft: 160 }}>
            Project Information
          </Subheader>
          <div className="generic-settings__label-with-input">
            <p className="generic-settings__label">
              <span>Title</span>
              <TagCopier content="${project-title}" />
            </p>
            <Input
              type="text"
              value={engine.title}
              onChange={(value) => (engine.title = value)}
            />
          </div>
          <div className="generic-settings__label-with-input">
            <p className="generic-settings__label">
              <span>Subtitle</span>
              <TagCopier content="${project-subtitle}" />
            </p>
            <Input
              type="text"
              value={engine.subtitle}
              onChange={(value) => (engine.subtitle = value)}
            />
          </div>
          <div className="generic-settings__label-with-input">
            <p className="generic-settings__label">
              <span>Composer</span>
              <TagCopier content="${project-composer}" />
            </p>
            <Input
              type="text"
              value={engine.composer}
              onChange={(value) => (engine.composer = value)}
            />
          </div>
          <div className="generic-settings__label-with-input">
            <p className="generic-settings__label">
              <span>Arranger</span>
              <TagCopier content="${project-arranger}" />
            </p>
            <Input
              type="text"
              value={engine.arranger}
              onChange={(value) => (engine.arranger = value)}
            />
          </div>
          <div className="generic-settings__label-with-input">
            <p className="generic-settings__label">
              <span>Lyricist</span>
              <TagCopier content="${project-lyricist}" />
            </p>
            <Input
              type="text"
              value={engine.lyricist}
              onChange={(value) => (engine.lyricist = value)}
            />
          </div>
          <div className="generic-settings__label-with-input">
            <p className="generic-settings__label">
              <span>Copyright</span>
              <TagCopier content="${project-copyright}" />
            </p>
            <Textarea
              value={engine.copyright}
              onChange={(value) => (engine.copyright = value)}
            />
          </div>
        </div>
      </div>

      <div className="generic-settings__buttons">
        <div className="generic-settings__spacer" />
        <Button compact onClick={onClose}>
          Close
        </Button>
      </div>
    </div>
  );
});
