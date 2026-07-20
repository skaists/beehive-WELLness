// Rendered-contrast probe for WCAG 2.1 AA — run in a browser console/JS tool against a live surface.
// FIX over the v1 probe: v1 read `color(srgb r g b)` computed values (0-1 floats) as 0-255 ints,
// so any element with a color-mix() background composited to near-black rgb(1,1,1) and reported a
// phantom 1.01:1 "dark-on-dark" failure. That bug inflated the F-4 audit and invented dark-on-dark
// on the light module surfaces. This parser handles rgb()/rgba()/color(srgb ...) correctly.
// Positive controls run first: the checker MUST flag a known-bad pair before its passes are believed.
(() => {
  const lin = c => { c /= 255; return c <= 0.03928 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4); };
  const lum = ([r, g, b]) => 0.2126 * lin(r) + 0.7152 * lin(g) + 0.0722 * lin(b);
  const parse = s => {
    s = (s || '').trim();
    const cm = s.match(/color\(\s*srgb\s+([\d.]+)\s+([\d.]+)\s+([\d.]+)(?:\s*\/\s*([\d.]+))?/i);
    if (cm) return { rgb: [+cm[1], +cm[2], +cm[3]].map(x => Math.round(x * 255)), a: cm[4] !== undefined ? +cm[4] : 1 };
    const m = s.match(/[\d.]+/g); if (!m) return null;
    const n = m.map(Number); return { rgb: n.slice(0, 3), a: n.length > 3 ? n[3] : 1 };
  };
  const over = (f, a, b) => f.map((c, i) => Math.round(c * a + b[i] * (1 - a)));
  const bgOf = e => {
    const st = []; let n = e;
    while (n && n !== document.documentElement) { const p = parse(getComputedStyle(n).backgroundColor); if (p && p.a > 0) st.push(p); n = n.parentElement; }
    const r = parse(getComputedStyle(document.documentElement).backgroundColor);
    let b = (r && r.a > 0) ? r.rgb : [255, 255, 255];
    for (let i = st.length - 1; i >= 0; i--) b = over(st[i].rgb, st[i].a, b);
    return b;
  };
  const R = (a, b) => { const L1 = lum(a), L2 = lum(b); return (Math.max(L1, L2) + 0.05) / (Math.min(L1, L2) + 0.05); };
  // POSITIVE CONTROLS — prove the checker bites before trusting a pass.
  const pc = {
    bad_777_on_white: +R([119, 119, 119], [255, 255, 255]).toFixed(2), // 4.48 — MUST be < 4.5
    good_black_on_white: +R([0, 0, 0], [255, 255, 255]).toFixed(2),    // 21   — MUST pass
  };
  const pcOk = pc.bad_777_on_white < 4.5 && pc.good_black_on_white >= 4.5;
  const texts = [...document.querySelectorAll('body *')].filter(e =>
    [...e.childNodes].some(n => n.nodeType === 3 && n.textContent.trim().length > 1) && e.offsetParent !== null);
  const fails = [];
  texts.forEach(e => {
    const cs = getComputedStyle(e); const f = parse(cs.color); const bg = bgOf(e);
    const fg = f.a < 1 ? over(f.rgb, f.a, bg) : f.rgb;
    const px = parseFloat(cs.fontSize); const bold = (parseInt(cs.fontWeight) || 400) >= 700;
    const need = (px >= 24 || (bold && px >= 18.66)) ? 3 : 4.5;
    const r = R(fg, bg);
    if (r < need) fails.push({
      sel: e.tagName.toLowerCase() + '.' + ([...e.classList].join('.') || '?'),
      fg: cs.color, bg: 'rgb(' + bg.join(',') + ')', ratio: +r.toFixed(2), need,
      txt: e.textContent.trim().slice(0, 24),
    });
  });
  const by = {}; fails.forEach(f => { const k = f.fg + ' on ' + f.bg; by[k] = (by[k] || 0) + 1; });
  return { title: document.title, positive_control_ok: pcOk, pc, total: fails.length, by, fails };
})()
