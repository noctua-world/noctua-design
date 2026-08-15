// Filling a sentence a caller wrote, and choosing the plural form of one.
//
// NEVER ASSEMBLING ONE. `NOCTUA.md §9.9`'s rule that matters most is that a
// sentence is one message: word order, gender and agreement all differ between
// languages, and a fragment that reads correctly in English can be
// ungrammatical once translated. A source gate enforces it in Rust. What is
// left for a script is putting a value into a hole the translator left, which
// is what these two do and all they do.
//
// It moved here out of `color-picker.js` when `inputs.js` turned out to need
// exactly the same two functions. Two copies of a plural rule is one of them
// being wrong in a language nobody on the team reads.

/**
 * A sentence with `{holes}` in it, filled.
 *
 * An absent sentence fills to nothing rather than to `undefined`: a caller who
 * did not supply a label gets no text, not the word *undefined* in their
 * interface.
 */
export function fill(sentence, holes) {
  if (!sentence) return "";
  return Object.entries(holes).reduce(
    (text, [key, value]) => text.split(`{${key}}`).join(value),
    sentence,
  );
}

/**
 * The plural form a count needs, from `one:…|other:…` or a plain sentence.
 *
 * `Intl.PluralRules` rather than `count === 1`, because that test is wrong in
 * most languages and silently wrong in all of them: Portuguese and English
 * agree, and Polish has three forms and Arabic six. The locale comes from the
 * document, which is where `NOCTUA.md §9.9` puts it.
 *
 * A sentence with no `:` in it is a sentence the caller chose not to inflect,
 * and it is filled rather than refused — *{count} selected* is a legitimate
 * thing to write.
 */
export function plural(sentence, count) {
  if (!sentence) return "";
  if (!sentence.includes(":")) return fill(sentence, { count });
  const forms = new Map(
    sentence.split("|").map((part) => {
      const at = part.indexOf(":");
      return [part.slice(0, at).trim(), part.slice(at + 1)];
    }),
  );
  const rule = new Intl.PluralRules(document.documentElement.lang || undefined).select(count);
  return fill(forms.get(rule) ?? forms.get("other") ?? "", { count });
}

/**
 * Puts one sentence in a live region, and never the same one twice running.
 *
 * Replacing text with identical text announces nothing in most screen readers,
 * so a count that has not changed is written quietly and an event is written
 * loudly — which is the difference between a region that speaks when something
 * happened and one a reader turns off.
 */
export function say(region, sentence, { quiet = false } = {}) {
  if (!region || !sentence) return;
  if (quiet && region.textContent === sentence) return;
  region.textContent = sentence;
}
