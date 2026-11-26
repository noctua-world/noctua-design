// A Meter's or a Progress's line, read the way a reader reads it: the name at
// the start, and the reading beside it at the end.
//
// THE LINE WRAPS, so a narrow column can put the reading under the name instead
// of pushing it past the line's end. What it must not do is wrap where nothing
// forces it to. With the name growing from nothing (`flex: 1 1 0`) and the
// reading growing as well, the two split what the line left evenly, so the
// name got only half of it and broke into two or three lines beside a reading
// box that was mostly empty: `Em cache` in two lines at 1280px, on a line 226px
// wide holding 94px of reading. No other spec could see it. The meaning and the
// colour specs never measure the line, and the narrow walk in
// `responsive.spec.js` asks only that nothing is pushed past a row's end, which
// a wrapped name never is.
//
// So the claim is put the way a reader would: where the name and the reading,
// each on one line at its widest, fit side by side with the gap between them,
// they are drawn so. The name takes one line, and the reading stands on it.

/**
 * The viewport widths a line is measured at: the floor every page is held to,
 * the phones above it, the column either side of the sidebar docking, and a
 * desktop, where the defect this guards against showed most.
 */
export const LINE_WIDTHS = [320, 360, 400, 500, 768, 1280];

/**
 * Every `.nd-<kind>-line` on the page, measured at the page's present width.
 *
 * `fitted` counts the lines where the name and the reading fit side by side at
 * their widest, which is the floor a caller asserts on: a check that measured
 * no such line proved nothing. `broken` names each of those that was not drawn
 * on one line, with what was measured.
 *
 * A line whose name is off the screen (`hide_label`), or whose reading is not
 * drawn (a busy bar has none), has nothing to set side by side and is skipped,
 * as is a line not drawn at all.
 */
export async function sideBySide(page, kind) {
  return page.locator(`.nd-${kind}-line`).evaluateAll((lines, kind) => {
    const result = { fitted: 0, broken: [] };
    for (const line of lines) {
      const name = line.querySelector(`:scope > .nd-${kind}-label`);
      const reading = line.querySelector(`:scope > .nd-${kind}-value`);
      if (!name || !reading || line.getClientRects().length === 0) continue;
      if (reading.getClientRects().length === 0) continue;
      if (getComputedStyle(name).position === "absolute") continue;

      const style = getComputedStyle(line);
      const room =
        line.clientWidth -
        Number.parseFloat(style.paddingInlineStart) -
        Number.parseFloat(style.paddingInlineEnd);
      const gap = Number.parseFloat(style.columnGap);

      // ONE LINE OF IT, AT ITS WIDEST: a copy in the same line, so it is drawn
      // by the same rules in the same face, and taken out of the flow, so the
      // line it measures is the line as it stands.
      const widest = (element) => {
        const copy = element.cloneNode(true);
        copy.removeAttribute("id");
        copy.removeAttribute("for");
        Object.assign(copy.style, {
          position: "absolute",
          visibility: "hidden",
          inlineSize: "max-content",
          whiteSpace: "nowrap",
        });
        line.append(copy);
        const { width, height } = copy.getBoundingClientRect();
        copy.remove();
        return { width, height };
      };
      const one = widest(name);
      const other = widest(reading);
      // A pixel short of the line, so a pair that fits only by a fraction of
      // one is not asked to: that is the engine's rounding, not the rule.
      if (one.width + gap + other.width > room - 1) continue;
      result.fitted += 1;

      const drawn = name.getBoundingClientRect();
      const beside = reading.getBoundingClientRect();
      const said = `"${name.textContent.trim()} | ${reading.textContent.trim()}": the line is ${Math.round(room)}px, the name ${Math.round(one.width)}px, the reading ${Math.round(other.width)}px, and`;
      if (drawn.height > one.height + 1) {
        result.broken.push(
          `${said} the name is ${Math.round(drawn.width)}px wide in ${Math.round(drawn.height / one.height)} lines`,
        );
      } else if (beside.top >= drawn.bottom) {
        result.broken.push(`${said} the reading went under it`);
      }
    }
    return result;
  }, kind);
}
