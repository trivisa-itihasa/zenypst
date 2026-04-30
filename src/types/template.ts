/** A Typst document template. */
export interface Template {
  id: string;
  name: string;
  description: string;
  content: string;
  builtIn?: boolean;
}

/** A template from Typst Universe (packages.typst.org). */
export interface UniverseTemplate {
  name: string;
  version: string;
  description?: string;
  authors: string[];
  categories: string[];
  entrypoint: string;
}
