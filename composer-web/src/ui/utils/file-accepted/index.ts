function stepsMatch(acceptStep: string, typeStep: string) {
  return acceptStep === "*" || acceptStep === typeStep;
}

function pathsMatch(acceptPath: string[], typePath: string[]): boolean {
  for (let i = 0; i < acceptPath.length; i++) {
    const match = stepsMatch(acceptPath[i], typePath[i]);
    if (!match) {
      return false;
    }
  }
  return true;
}

/**
 *  Checks if a file type is accepted. Always return true if no accept param.
 */
export function fileAccepted(file: File, accept: string[] = []): boolean {
  // no accept length implies all accepted
  if (accept.length === 0) {
    return true;
  }

  const typePath = file.type.split("/");

  for (let i = 0; i < accept.length; i++) {
    const acceptPath = accept[i].split("/");
    const match = pathsMatch(acceptPath, typePath);
    if (match) {
      // return true if any match
      return true;
    }
  }

  return false;
}
