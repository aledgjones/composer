import { engine, store } from ".";
import { wait } from "../ui/utils/wait";

const writeToFile = async (state: any, fileHandle: FileSystemFileHandle) => {
  const content = JSON.stringify(state);
  const writableStream = await fileHandle.createWritable();
  await writableStream.write(content);
  await writableStream.close();
};

export const appActions = {
  open: async (progress: (progress: number, total: number) => void) => {
    progress(0, 2);
    const pickerOpts = {
      types: [
        {
          description: "Scores",
          accept: {
            "application/scf": [".scf"],
          },
        },
      ],
      excludeAcceptAllOption: true,
      multiple: false,
    };

    // open file picker
    const [fileHandle] = await window.showOpenFilePicker(pickerOpts);
    const file = await fileHandle.getFile();
    const content = await file.text();
    engine.import(JSON.parse(content));
    progress(1, 2);
    store.update((s) => {
      s.app.file = fileHandle;
    });
    await wait(1000);
    progress(2, 2);
  },
  save: async () => {
    try {
      const state = store.getRawState();
      if (state.app.file) {
        await writeToFile(engine.export(), state.app.file);
      } else {
        throw "no file";
      }
    } catch (e) {
      console.log(e);
    }
  },
  saveAs: async () => {
    try {
      const newHandle = await window.showSaveFilePicker({
        suggestedName: "untitled.scf",
        types: [
          {
            description: "Scores",
            accept: {
              "application/scf": [".scf"],
            },
          },
        ],
        excludeAcceptAllOption: true,
      });
      await writeToFile(engine.export(), newHandle);
      store.update((s) => {
        s.app.file = newHandle;
      });
    } catch (e) {
      console.log(e);
    }
  },
};
