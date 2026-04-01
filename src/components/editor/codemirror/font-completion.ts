import {
  autocompletion,
  completionKeymap,
  type Completion,
  type CompletionContext,
  type CompletionSource,
} from "@codemirror/autocomplete";
import { keymap } from "@codemirror/view";
import type { Extension } from "@codemirror/state";

/**
 * Returns a CodeMirror extension that suggests font names when the cursor is
 * inside a quoted string in a Typst font context
 * (e.g. `#set text(font: "▌")` or `#set text(font: ("▌", ...))`).
 *
 * `getFonts` is called on every completion request so the list can be
 * updated asynchronously after the extension is created.
 */
export function fontCompletion(getFonts: () => string[]): Extension {
  const source: CompletionSource = (context: CompletionContext) => {
    const { state, pos } = context;

    // Look back up to 300 characters to handle multi-line cases like:
    //   #set text(font:
    //     "▌")
    const lookback = Math.max(0, pos - 300);
    const textBefore = state.sliceDoc(lookback, pos);

    // Walk through to find the last unescaped `"` that opens a string
    let inString = false;
    let openQuoteIdx = -1;
    for (let i = 0; i < textBefore.length; i++) {
      if (textBefore[i] === '"') {
        inString = !inString;
        if (inString) openQuoteIdx = i;
      }
    }

    if (!inString || openQuoteIdx === -1) return null;

    // Only suggest if there is a `font` keyword before the opening quote
    const beforeQuote = textBefore.slice(0, openQuoteIdx);
    if (!/\bfont\b/i.test(beforeQuote)) return null;

    const typed = textBefore.slice(openQuoteIdx + 1);

    const options: Completion[] = getFonts().map((f) => ({
      label: f,
      type: "text",
    }));

    return {
      from: pos - typed.length,
      options,
      validFor: /^[^"]*$/,
    };
  };

  return [
    autocompletion({ override: [source], activateOnTyping: true }),
    keymap.of(completionKeymap),
  ];
}
