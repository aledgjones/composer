import Color from "color";

/**
 * Hook: sets the alpha value of a color.
 */
export function useAlpha(color: string, alpha: number) {
  return Color(color).alpha(alpha).string();
}
