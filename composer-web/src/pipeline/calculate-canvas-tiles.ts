interface Tile {
  key: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

const TILE_SIZE = 400;

export const calculateCanvasTiles = (width: number, height: number) => {
  const output: Tile[] = [];

  const columns = Math.ceil(width / TILE_SIZE);
  const rows = Math.ceil(height / TILE_SIZE);

  const columnWidth = TILE_SIZE;
  const rowHeight = TILE_SIZE;

  for (let y = 0; y < rows; y++) {
    for (let x = 0; x < columns; x++) {
      output.push({
        key: `${x},${y}`,
        x: x * columnWidth,
        y: y * rowHeight,
        width: columnWidth,
        height: rowHeight,
      });
    }
  }

  return output;
};
