/**
 * Super simple pluralizer convenience method.
 */
export function usePluralize(num: number, single: string, plural: string) {
  return num === 1 ? single : plural;
}
