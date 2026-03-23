/** A node in the file tree (file or directory). */
export interface FileNode {
  name: string;
  path: string;
  isDir: boolean;
  children?: FileNode[];
  extension?: string;
}

/** A tab representing an open file in the editor. */
export interface FileTab {
  id: string;
  path: string | null;
  name: string;
  content: string;
  isDirty: boolean;
  isUntitled: boolean;
}

/** A filter for file picker dialogs. */
export interface FileFilter {
  name: string;
  extensions: string[];
}
