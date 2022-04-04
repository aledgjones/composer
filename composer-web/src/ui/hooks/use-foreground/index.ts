import Color from "color";

/**
 * Hook: returns white on dark colors, black on light;
 */
export function useForeground(color: string) {
  return Color(color).isDark() ? "#ffffff" : "#000000";
}
