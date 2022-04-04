/**
 * Checks if an email is valid
 */
export function isEmail(email: string) {
  // from [https://emailregex.com/](https://emailregex.com/)
  const regex = /^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$/;
  return regex.test(email);
}
