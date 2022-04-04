export interface Error {
  code: string;
  message: string;
}

/**
 * Generate an error object, useful for throwing inside a catch block.
 */
export function error(
  code: string = "@general/unknown-error",
  message: string = "Something went wrong. Please try again."
): Error {
  return { code, message };
}
