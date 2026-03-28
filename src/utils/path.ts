export function getDirectory(filePath: string): string {
  const sep = filePath.includes("/") ? "/" : "\\";
  const parts = filePath.split(sep);
  parts.pop();
  return parts.join(sep) || "/";
}

export function ensureTypExtension(name: string): string {
  return name.includes(".") ? name : name + ".typ";
}
