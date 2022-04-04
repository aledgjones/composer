export function getAbsoluteWidth(element: HTMLDivElement | null) {
  if (element) {
    var styles = window.getComputedStyle(element);
    var margin = parseFloat(styles["marginRight"]);
    return Math.ceil(element.offsetWidth + margin);
  } else {
    return 0;
  }
}
