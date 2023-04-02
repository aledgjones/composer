import { engine, store } from ".";
import { wait } from "../ui/utils/wait";

//** convert maps into JSON objects */
function replacer(key: string, value: any) {
  if (value instanceof Map) {
    return {
      dataType: "Map",
      value: Array.from(value.entries()), // or with spread: value: [...value]
    };
  } else {
    return value;
  }
}

//** convert stored map objects back to maps */
function reviver(key: string, value: any) {
  if (typeof value === "object" && value !== null) {
    if (value.dataType === "Map") {
      return new Map(value.value);
    }
  }
  return value;
}

const writeToFile = async (state: any, fileHandle: FileSystemFileHandle) => {
  const content = JSON.stringify(state, replacer, 2);
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
    engine.import(JSON.parse(content, reviver));
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
        const score = engine.export();
        await writeToFile(score, state.app.file);
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
      const score = engine.export();
      await writeToFile(score, newHandle);
      store.update((s) => {
        s.app.file = newHandle;
      });
    } catch (e) {
      console.log(e);
    }
  },
};
