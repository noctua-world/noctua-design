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
 * The two above, in the order every caller wants them.
 *
 * A caller filling a sentence that counts something has to choose the form
 * BEFORE putting the values in, or `{count}` is substituted into a template
 * that still holds every variant. Four enhancers each wrote that composition
 * out; the documentation site's own script wrote a fifth copy with its own
 * plural rule, which is the thing this file's header refuses.
 *
 * `count` of `null` means the sentence counts nothing and is only filled.
 */
export function message(sentence, holes = {}, count = null) {
  return fill(count === null ? sentence : plural(sentence, count), holes);
}

/**
 * Puts one sentence in a live region, and never the same one twice running.
 *
 * Replacing text with identical text announces nothing in most screen readers,
 * so a count that has not changed is written quietly and an event is written
 * loudly — which is the difference between a region that speaks when something
 * happened and one a reader turns off.
 *
 * NOTHING IS A SENTENCE TOO, AND IT USED TO BE THE ONE THIS REFUSED. An empty
 * string returned here early, so every caller clearing a refusal left the old
 * refusal on the screen: `TimeInput` and `OtpField` both say nothing when the
 * value became valid, and both went on showing the complaint about the value
 * before it. A region that cannot be emptied is a region that only ever grows.
 *
 * The guard that remains is the one it was written for: an absent region.
 */
export function say(region, sentence, { quiet = false } = {}) {
  if (!region) return;
  const text = sentence ?? "";
  if (quiet && region.textContent === text) return;
  region.textContent = text;
}

/**
 * The element inside a field's error paragraph that holds the sentence.
 *
 * `Field` renders a visually hidden `error_label` in front of the message —
 * the word *Error*, in the reader's language — and every script that reports a
 * refusal used to write `slot.textContent`, which deleted it. So the one case
 * `error_slot` exists for, a browser's own complaint with no author error
 * beside it, was the one case that never got its prefix. The message has an
 * element of its own now, and this is the only function that finds it.
 */
export function messageIn(slot) {
  return slot?.querySelector("[data-nd-field-message]") ?? slot ?? null;
}

/** Puts a sentence into a field's error paragraph, prefix untouched. */
export function saySentence(slot, sentence) {
  const into = messageIn(slot);
  if (into) into.textContent = sentence;
}

/** What that paragraph says, without the hidden word in front of it. */
export function sentenceOf(slot) {
  return messageIn(slot)?.textContent.trim() ?? "";
}
