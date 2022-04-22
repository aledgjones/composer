export const timer = (name: string, active: boolean, fn: () => any) => {
  if (active) {
    performance.mark(`start-${name}`);
  }

  const result = fn();

  if (active) {
    performance.measure(name, `start-${name}`);
    const entries = performance.getEntriesByType("measure");
    for (let i = 0; i < entries.length; i++) {
      const entry = entries[i];
      console.log(
        `${entry.name}: %c${entry.duration}`,
        (entry.duration < 1000 / 60 && "color: green") ||
          (entry.duration < 1000 / 30 && "color: orange") ||
          "color: red"
      );
    }
    performance.clearMarks();
    performance.clearMeasures();
  }

  return result;
};
