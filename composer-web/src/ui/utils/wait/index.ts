/**
 * Generate a Promise that will be resolved after specified duration.
 */
export async function wait(duration = 1000) {
  return new Promise((resolve) => {
    setTimeout(resolve, duration);
  });
}
