function reverseString(string) {
  if (string.length < 2 || typeof string !== "string") {
    return string;
  }

  let first = 0;
  let last = string.length - 1;
  stringArray = string.split('');
  while (first < last) {
    const value = stringArray[first];
    stringArray[first] = stringArray[last];
    stringArray[last] = value;
    first++;
    last--;
  }

  return stringArray.toString();
}
