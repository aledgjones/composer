import { error } from "../error";

export async function download(blob: any, name: string): Promise<void> {
  const localUrl = URL.createObjectURL(blob);

  // yes we need to append else it doesn't work
  const a: any = document.createElement("A");
  document.body.appendChild(a);
  a.style.position = "fixed";
  a.style.visibility = "hidden";
  a.style.left = "-100000px";
  a.style.top = "-100000px";
  a.href = localUrl;
  a.download = name;

  try {
    a.click();
    a.remove();
    URL.revokeObjectURL(localUrl);
    return;
  } catch (err) {
    a.remove();
    URL.revokeObjectURL(localUrl);
    throw error("@download/fail", "Could not download the file.");
  }
}
