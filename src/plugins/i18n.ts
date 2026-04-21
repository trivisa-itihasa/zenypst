import { createI18n } from "vue-i18n";
import {
  compile,
  resolveValue,
  fallbackWithLocaleChain,
} from "@intlify/core-base";
import en from "@/locales/en";

const i18n = createI18n({
  legacy: false,
  locale: "en",
  fallbackLocale: "en",
  messages: { en },
  // Pass the message compiler, resolver, and locale fallbacker directly so
  // they survive tree-shaking.  The vue-i18n entry normally registers these via
  // module-level side effects, but every @intlify package is marked
  // sideEffects:false so Vite's production build strips those calls.
  messageCompiler: compile,
  messageResolver: resolveValue,
  localeFallbacker: fallbackWithLocaleChain,
});

export default i18n;
