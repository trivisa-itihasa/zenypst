/** A Typst document template. */
export interface Template {
  id: string;
  name: string;
  description: string;
  content: string;
  builtIn?: boolean;
}
