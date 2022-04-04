/**
 * Copy a string to the clipboard
 */
export async function copy(str: string): Promise<void> {
  try {
    await navigator.clipboard.writeText(str);
    return Promise.resolve();
  } catch (e) {
    const input: any = document.createElement("input");
    input.style.opacity = 0;
    input.value = str;
    document.body.appendChild(input);
    input.select();
    try {
      document.execCommand("copy");
      input.remove();
      return Promise.resolve();
    } catch (err) {
      input.remove();
      return Promise.reject();
    }
  }
}
