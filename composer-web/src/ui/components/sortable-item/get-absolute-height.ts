export function getAbsoluteHeight(element: HTMLDivElement | null) {
  if (element) {
    var styles = window.getComputedStyle(element);
    var margin = parseFloat(styles["marginBottom"]);
    return Math.ceil(element.offsetHeight + margin);
  } else {
    return 0;
  }
}
