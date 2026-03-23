import { StreamLanguage, LanguageSupport } from "@codemirror/language";

/**
 * Basic Typst syntax highlighting using StreamLanguage (simpler than full Lezer grammar).
 * Covers: headings, bold, italic, math, raw, comments, keywords, functions, labels, references.
 */
const typstStreamLanguage = StreamLanguage.define({
  name: "typst",

  startState() {
    return {
      inMath: false,
      inRaw: false,
      inRawBlock: false,
      mathDepth: 0,
    };
  },

  token(stream, state) {
    // Raw block (triple backtick)
    if (stream.match("```")) {
      state.inRawBlock = !state.inRawBlock;
      if (state.inRawBlock) {
        stream.skipToEnd();
      }
      return "meta"; // rawBlock
    }
    if (state.inRawBlock) {
      stream.skipToEnd();
      return "meta";
    }

    // Raw inline (single backtick)
    if (stream.match("`")) {
      state.inRaw = !state.inRaw;
      return "meta";
    }
    if (state.inRaw) {
      if (stream.skipTo("`")) {
        stream.next();
        state.inRaw = false;
      } else {
        stream.skipToEnd();
      }
      return "meta";
    }

    // Comments
    if (stream.match("//")) {
      stream.skipToEnd();
      return "lineComment";
    }
    if (stream.match("/*")) {
      if (!stream.skipTo("*/")) {
        stream.skipToEnd();
      } else {
        stream.match("*/");
      }
      return "blockComment";
    }

    // Math mode
    if (stream.match("$")) {
      state.inMath = !state.inMath;
      return "keyword"; // math delimiter
    }
    if (state.inMath) {
      if (stream.skipTo("$")) {
        stream.next();
        state.inMath = false;
      } else {
        stream.skipToEnd();
      }
      return "number"; // math content
    }

    // Headings (= at start of line)
    if (stream.sol() && stream.match(/^=+\s/)) {
      stream.skipToEnd();
      return "heading";
    }

    // Labels <label>
    if (stream.match(/<[a-zA-Z0-9_:-]+>/)) {
      return "link";
    }

    // References @ref
    if (stream.match(/@[a-zA-Z0-9_:-]+/)) {
      return "link";
    }

    // Hash expressions: keywords and functions
    if (stream.peek() === "#") {
      stream.next();
      // Check for keywords
      const keywords = [
        "set", "let", "show", "import", "include",
        "if", "else", "for", "while", "return", "break", "continue",
        "none", "true", "false", "not", "and", "or", "in",
      ];
      for (const kw of keywords) {
        if (stream.match(kw) && !stream.match(/[a-zA-Z0-9_]/)) {
          return "keyword";
        }
      }
      // Function call
      if (stream.match(/[a-zA-Z_][a-zA-Z0-9_]*/)) {
        return "variableName";
      }
      return null;
    }

    // Strings
    if (stream.match('"')) {
      while (!stream.eol()) {
        if (stream.next() === '"') break;
      }
      return "string";
    }

    // Numbers
    if (stream.match(/^\d+(\.\d+)?(%|pt|mm|cm|in|em|rem|fr)?/)) {
      return "number";
    }

    // Bold: *text*
    if (stream.match("*")) {
      return "strong";
    }

    // Italic: _text_
    if (stream.match("_")) {
      return "emphasis";
    }

    // Operators
    if (stream.match(/^[+\-\/*=<>!&|^~]+/)) {
      return "operator";
    }

    // Brackets
    if (stream.match(/^[(){}\[\]]/)) {
      return "bracket";
    }

    stream.next();
    return null;
  },

  languageData: {
    commentTokens: { line: "//" },
  },
});

/** Create a Typst LanguageSupport instance. */
export function typstLanguage(): LanguageSupport {
  return new LanguageSupport(typstStreamLanguage);
}
